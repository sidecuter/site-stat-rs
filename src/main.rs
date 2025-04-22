use actix_web::web::{Data, JsonConfig, QueryConfig};
use sea_orm::ConnectOptions;
use sea_orm::{Database, DatabaseConnection};
use stat_api::config::AppConfig;
use stat_api::cors::create_cors;
use stat_api::mut_state::AppStateMutable;
use stat_api::task::start_data_refresh_task;
use stat_api::{api, api_docs, errors::ApiError};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    init_logger();
    let config = Data::new(AppConfig::default());
    let addr = config.get_addr();
    let pool = Data::new(get_database_connection(&config.database_url).await);
    let files_path = config.get_files_path();
    let front_path = config.get_front_path();
    ensure_dir_exists(&files_path)?;
    ensure_dir_exists(&front_path)?;
    let state = Data::new(AppStateMutable::default());

    tracing::info!("Listening on http://{addr}");
    tracing::info!("Redoc UI is available at http://{addr}/redoc");

    actix_rt::spawn(start_data_refresh_task(
        state.clone(),
        std::time::Duration::from_secs(config.data_refresh_interval),
    ));

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(create_cors(&config))
            .app_data(state.clone())
            .app_data(pool.clone())
            .app_data(config.clone())
            .wrap(actix_web::middleware::Logger::default())
            .configure(api::init_routes)
            .app_data(json_errors_handler())
            .app_data(query_errors_handler())
            .service(Redoc::with_url("/redoc", api_docs::ApiDoc::openapi()))
            .service(frontend(front_path.to_str().unwrap()))
    })
    .bind(addr)?
    .run()
    .await
}

async fn get_database_connection(connection_string: &str) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(connection_string);
    if cfg!(not(debug_assertions)) {
        opt.sqlx_logging(false);
    }
    Database::connect(opt)
        .await
        .expect("Failed to create database connection pool")
}

fn init_logger() {
    let level = if cfg!(debug_assertions) {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };
    tracing_subscriber::fmt()
        .with_max_level(level) // В продакшене использовать JSON формат
        .init();
}

fn ensure_dir_exists(path: &std::path::Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir(path)?;
    }
    Ok(())
}

fn json_errors_handler() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _| ApiError::from(err).into())
}

fn query_errors_handler() -> QueryConfig {
    QueryConfig::default().error_handler(|err, _| ApiError::from(err).into())
}

fn frontend(path: &str) -> actix_files::Files {
    actix_files::Files::new("/", path).index_file("index.html")
}
