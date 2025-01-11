use stat_api::{
    api_docs, errors::Error as ApiError,
    app_state::AppState,
    api,
};
use actix_web::{
    self,
    middleware::Logger,
    web::{self, JsonConfig, QueryConfig},
    App, HttpRequest, HttpServer
};
use sea_orm::{Database, DatabaseConnection};
#[cfg(not(debug_assertions))]
use sea_orm::ConnectOptions;
use utoipa_swagger_ui::SwaggerUi;

#[cfg(not(debug_assertions))]
async fn get_database_connection(connection_string: &str) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(connection_string);
    opt.sqlx_logging(false);
    Database::connect(opt).await.expect("Failed to create database connection pool")
}

#[cfg(debug_assertions)]
async fn get_database_connection(connection_string: &str) -> DatabaseConnection {
    Database::connect(connection_string).await.expect("Failed to create database connection pool")
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
    }
    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .init();
    }

    let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_owned());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let addr = format!("{host}:{port}");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://app.db?mode=rwc".to_owned());
    let admin_key = std::env::var("ADMIN_KEY").unwrap_or_else(|_| "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_owned());
    let pool = get_database_connection(&database_url).await;

    log::info!("Listening on http://{}", addr);
    log::info!(
        "OpenAPI document is available at http://{}/docs/openapi.json",
        addr,
    );
    log::info!("Swagger UI is available at http://{}/docs/swagger/", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(AppState{admin_key: admin_key.clone()}))
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
