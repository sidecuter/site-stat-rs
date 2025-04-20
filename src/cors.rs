use crate::app_state::AppState;
use actix_cors::Cors;
use actix_web::web;

pub fn create_cors(app_state: &web::Data<AppState>) -> Cors {
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
    cors
}
