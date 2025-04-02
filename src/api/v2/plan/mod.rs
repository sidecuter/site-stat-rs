use actix_web::web;

mod get;
mod add;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/plan")
            .service(get::get_plans)
            .service(add::add_stat_plan)
    );
}