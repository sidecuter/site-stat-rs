use actix_web::web;

pub mod user_id;
pub mod site;
pub mod aud;
pub mod way;
pub mod plan;
pub mod stat;
pub mod review;
pub mod popular;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v2")
            .configure(user_id::init_routes)
            .configure(site::init_routes)
            .configure(aud::init_routes)
            .configure(way::init_routes)
            .configure(plan::init_routes)
            .configure(stat::init_routes)
            .configure(review::init_routes)
            .configure(popular::init_routes)
    );
}
