use crate::config::AppConfig;
use actix_cors::Cors;
use actix_web::http::Method;
use actix_web::web;
use std::str::FromStr;

pub fn create_cors(config: &web::Data<AppConfig>) -> Cors {
    let allowed_origins = config.allowed_hosts.as_deref().unwrap_or_default();
    let allowed_methods = config.allowed_methods.as_deref().unwrap_or_default();

    allowed_origins
        .iter()
        .filter(|host| !host.is_empty())
        .fold(Cors::default(), |cors, host| cors.allowed_origin(host))
        .allowed_methods(
            allowed_methods
                .iter()
                .filter_map(|method| Method::from_str(method).ok()),
        )
}
