pub mod site;
pub mod aud;
pub mod way;
pub mod plan;

use actix_web::web;
use self::way::stat_way;
use self::site::stat_site;
use self::aud::stat_aud;
use self::plan::stat_plan;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stat")
            .service(stat_site)
            .service(stat_aud)
            .service(stat_way)
            .service(stat_plan)
    );
}
