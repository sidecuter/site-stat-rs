use std::fs;
use actix_web::{
    self,
    middleware::Logger,
    web::{self, JsonConfig, QueryConfig},
    App, HttpServer,
};
use actix_cors::Cors;
#[cfg(not(debug_assertions))]
use sea_orm::ConnectOptions;
use sea_orm::{Database, DatabaseConnection};
use utoipa::OpenApi;
use stat_api::{api, api_docs, errors::ApiError};
use utoipa_redoc::{Redoc, Servable};
use stat_api::app_state::AppState;

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
    let app_state = web::Data::new(AppState::default());
    let addr = format!("{}:{}", app_state.host, app_state.port);
    let pool = get_database_connection(&app_state.database_url).await;
    if !std::path::Path::new(&app_state.files_path).exists() {
        fs::create_dir(app_state.files_path.clone())?;
    }
    if !std::path::Path::new(&app_state.front_path).exists() {
        fs::create_dir(app_state.front_path.clone())?;
    }
    tracing::info!("Listening on http://{addr}");
    // tracing::info!(
    //     "OpenAPI document is available at http://{addr}/docs/openapi.json",
    // );
    tracing::info!("Redoc UI is available at http://{addr}/redoc");

    HttpServer::new(move || {
        let cors = Cors::default();
        let cors = if let Some(host) = &app_state.allowed_host {
            cors.allowed_origin(&host.clone())
        } else {
            cors
        };
        let cors = if let Some(methods) = &app_state.allowed_methods {
            cors.allowed_methods(methods.clone())
        } else {
            cors
        };
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .configure(api::init_routes)
            .app_data(JsonConfig::default().error_handler(|err, _| ApiError::from(err).into()))
            .app_data(QueryConfig::default().error_handler(|err, _| ApiError::from(err).into()))
            .service(Redoc::with_url("/redoc", api_docs::ApiDoc::openapi()))
            .service(actix_files::Files::new("/", &app_state.front_path).index_file("index.html"))
    })
        .bind(addr)?
        .run()
        .await
}
