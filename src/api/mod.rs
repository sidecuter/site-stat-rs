use actix_web::middleware::NormalizePath;
use actix_web::web;

pub mod get;
pub mod review;
pub mod stat;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(get::init_routes)
            .configure(stat::init_routes)
            .configure(review::init_routes)
            .wrap(NormalizePath::trim()),
    );
}
