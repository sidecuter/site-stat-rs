#[derive(Clone, Debug)]
pub struct AppState {
    pub host: String,
    pub port: String,
    pub admin_key: String,
    pub database_url: String,
    pub files_path: String,
}

impl AppState {
    pub fn new() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_owned());
        let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://app.db?mode=rwc".to_owned());
        let admin_key = std::env::var("ADMIN_KEY").unwrap_or_else(|_|
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_owned()
        );
        let files_path = std::env::var("FILES_PATH").unwrap_or_else(|_| "./static".to_owned());
        Self { host, port, admin_key, database_url, files_path }
    }
}
