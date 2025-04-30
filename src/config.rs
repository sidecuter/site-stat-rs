use config::Config;
use serde::Deserialize;
use std::path::{Path, PathBuf};

fn default_host() -> String {
    String::from("127.0.0.1")
}

fn default_port() -> String {
    String::from("8080")
}

fn default_admin_key() -> String {
    String::from("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
}

fn default_database_url() -> String {
    String::from("sqlite://app.db?mode=rwc")
}

fn default_static_path() -> String {
    String::from("./static")
}

fn default_files_dir() -> String {
    String::from("images")
}

fn default_front_dir() -> String {
    String::from("web")
}

fn default_refresh() -> u64 {
    600
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: String,
    #[serde(default = "default_admin_key")]
    pub admin_key: String,
    #[serde(default = "default_database_url")]
    pub database_url: String,
    #[serde(default = "default_static_path")]
    pub static_path: String,
    #[serde(default = "default_files_dir")]
    pub files_dir: String,
    #[serde(default = "default_front_dir")]
    pub front_dir: String,
    #[serde(default)]
    pub allowed_hosts: Option<Vec<String>>,
    #[serde(default)]
    pub allowed_methods: Option<Vec<String>>,
    #[serde(default = "default_refresh")]
    pub data_refresh_interval: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl AppConfig {
    pub fn new() -> Self {
        Config::builder()
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .list_separator(",")
                    .with_list_parse_key("allowed_hosts")
                    .with_list_parse_key("allowed_methods")
                    .ignore_empty(true),
            )
            .build()
            .expect("Expected valid config")
            .try_deserialize()
            .expect("Invalid config types")
    }

    pub fn get_base_path(&self) -> PathBuf {
        Path::new(&self.static_path).to_path_buf()
    }

    pub fn get_files_path(&self) -> PathBuf {
        Path::new(&self.static_path).join(&self.files_dir)
    }

    pub fn get_front_path(&self) -> PathBuf {
        Path::new(&self.static_path).join(&self.front_dir)
    }

    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
