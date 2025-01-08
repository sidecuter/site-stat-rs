use stat_api::{api_docs, errors::Error as ApiError};
use actix_web;
use actix_web::middleware::Logger;
use actix_web::{web, web::{JsonConfig, QueryConfig}, App, HttpRequest, HttpServer};
use sea_orm::Database;
use stat_api::api;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    // pretty_env_logger::init();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_owned());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let addr = format!("{host}:{port}");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://db.app".to_owned());
    let pool = Database::connect(database_url).await.expect("Failed to create database connection pool");

    log::info!("Listening on http://{}", addr);
    log::info!(
        "OpenAPI document is available at http://{}/docs/openapi.json",
        addr,
    );
    log::info!("Swagger UI is available at http://{}/docs/swagger/", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .configure(api::init_routes)
            .app_data(JsonConfig::default().error_handler(|err, _| ApiError::from(err).into()))
            .app_data(QueryConfig::default().error_handler(|err, _| ApiError::from(err).into()))
            .service(
                // OpenAPI document
                web::scope("/docs").service(api_docs::openapi_json).service(
                    SwaggerUi::new("/swagger/{_:.*}").url("/docs/openapi.json", Default::default()),
                ),
            )
            .default_service(web::route().to(|req: HttpRequest| async move {
                let path = req.path();
                if path.ends_with('/') {
                    ApiError::PathNotFound(format!(
                        "There is no endpoint in this path with this method. Our API doesn't support trailing slashes, try `{}`",
                        path.trim_end_matches('/')
                    ))
                } else {
                    ApiError::PathNotFound("There is no endpoint in this path with this method".to_owned())
                }
            }))
    })
        .bind(addr)?
        .run()
        .await
}
