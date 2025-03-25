use std::fs;
use actix_web::{
    self,
    middleware::Logger,
    web::{self, JsonConfig, QueryConfig},
    App, HttpServer,
};
#[cfg(not(debug_assertions))]
use sea_orm::ConnectOptions;
use sea_orm::{Database, DatabaseConnection};
use stat_api::{api, api_docs, app_state::AppState, errors::ApiError};
use utoipa_swagger_ui::SwaggerUi;

#[cfg(not(debug_assertions))]
async fn get_database_connection(connection_string: &str) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(connection_string);
    opt.sqlx_logging(false);
    Database::connect(opt)
        .await
        .expect("Failed to create database connection pool")
}

#[cfg(debug_assertions)]
async fn get_database_connection(connection_string: &str) -> DatabaseConnection {
    Database::connect(connection_string)
        .await
        .expect("Failed to create database connection pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }
    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }
    let app_state = AppState::new();
    let addr = format!("{}:{}", app_state.host, app_state.port);
    let pool = get_database_connection(&app_state.database_url).await;
    if !std::path::Path::new(&app_state.files_path).exists() {
        fs::create_dir(app_state.files_path.clone())?;
    }
    tracing::info!("Listening on http://{addr}");
    tracing::info!(
        "OpenAPI document is available at http://{addr}/docs/openapi.json",
    );
    tracing::info!("Swagger UI is available at http://{addr}/docs/swagger/");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(AppState::new()))
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
    })
        .bind(addr)?
        .run()
        .await
}
