use actix_web::web;

pub mod get;
pub mod add;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/site")
            .service(get::get_sites)
            .service(add::add_stat_site)
    );
}