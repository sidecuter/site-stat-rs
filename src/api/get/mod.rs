pub mod popular;
pub mod route;
pub mod stat;
pub mod user_id;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/get")
            .service(user_id::get_user_id)
            .service(stat::get_stat)
            .service(popular::get_popular)
            .service(route::get_route),
    );
}
