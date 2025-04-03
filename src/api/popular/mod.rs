use actix_web::web;

pub mod get;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/popular").service(get::get_popular)
    );
}