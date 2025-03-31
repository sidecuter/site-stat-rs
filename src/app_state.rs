use std::collections::HashSet;
use std::iter::Iterator;
use std::str::FromStr;
use std::string::ToString;
use std::sync::LazyLock;
use actix_web::http::Method;

#[derive(Clone, Debug)]
pub struct AppState {
    pub host: String,
    pub port: String,
    pub admin_key: String,
    pub database_url: String,
    pub files_path: String,
    pub allowed_host: Option<String>,
    pub allowed_methods: Option<Vec<Method>>,
}

const METHODS_ARRAY: [&str; 9] = [
    "GET",
    "PUT",
    "POST",
    "DELETE",
    "HEAD",
    "OPTIONS",
    "TRACE",
    "PATCH",
    "CONNECT"
];

static METHODS: LazyLock<HashSet<String>> = LazyLock::new(|| METHODS_ARRAY.iter().map(|&v| v.to_string()).collect());

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
        let allowed_host = std::env::var("ALLOWED_HOST").map_or(None, |v| Some(v));
        let allowed_methods = std::env::var("ALLOWED_METHODS").map_or(None, |v|
            Some(v
                .replace(['[', ']', '"', ' '], "")
                .split(",")
                .filter(|&v| METHODS.contains(v))
                .map(|v| Method::from_str(v).expect("Method should be allowed"))
                .collect()
            )
        );
        Self { host, port, admin_key, database_url, files_path, allowed_host, allowed_methods }
    }
}
