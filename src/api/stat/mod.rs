pub mod aud;
pub mod plan;
pub mod site;
pub mod way;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stat")
            .service(site::stat_site)
            .service(aud::stat_aud)
            .service(way::stat_way)
            .service(plan::stat_plan),
    );
}
