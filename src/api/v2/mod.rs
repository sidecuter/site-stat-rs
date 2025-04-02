use actix_web::web;

mod user_id;
mod site;
mod aud;
mod way;
mod plan;
mod stat;
mod review;
mod popular;

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
