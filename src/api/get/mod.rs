pub mod user_id;
pub mod sites;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/get")
            .service(user_id::get_user_id)
            .service(sites::get_sites)
    );
}