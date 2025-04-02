use actix_web::web;

mod get;
mod add;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/way")
            .service(get::get_ways)
            .service(add::add_stat_way)
    );
}
