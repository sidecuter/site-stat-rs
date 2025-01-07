use stat_api::api_docs;
use actix_web;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpServer};
use sea_orm::Database;
use anyhow::Result;
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
            // .default_service(web::route().to(|reg: HttpRequest| async move {
            //     let path = reg.path();
            //     if path.ends_with('/') {
            //         ApiErrr
            //     }
            // }))
            .service(
                // OpenAPI document
                web::scope("/docs").service(api_docs::openapi_json).service(
                    SwaggerUi::new("/swagger/{_:.*}").url("/docs/openapi.json", Default::default()),
                ),
            )
    })
        .bind(addr)?
        .run()
        .await
}
