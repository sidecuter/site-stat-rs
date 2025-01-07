pub mod site;

use actix_web::web;
use crate::api::stat::site::stat_site;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stat")
            .service(stat_site)
    );
}
