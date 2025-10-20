pub mod login;
pub mod me;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(login::login));
}
