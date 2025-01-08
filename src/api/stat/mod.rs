pub mod site;
pub mod aud;
mod way;

use actix_web::web;
use self::site::stat_site;
use self::aud::stat_aud;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stat")
            .service(stat_site)
            .service(stat_aud)
    );
}
