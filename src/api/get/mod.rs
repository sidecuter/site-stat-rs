pub mod auds;
pub mod plans;
pub mod popular;
pub mod route;
pub mod sites;
pub mod stat;
pub mod user_id;
pub mod ways;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/get")
            .service(user_id::get_user_id)
            .service(sites::get_sites)
            .service(auds::get_auds)
            .service(ways::get_ways)
            .service(plans::get_plans)
            .service(stat::get_stat)
            .service(popular::get_popular)
            .service(route::get_route),
    );
}
