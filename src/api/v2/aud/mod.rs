use actix_web::web;

mod get;
mod add;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/aud")
            .service(get::get_auds)
            .service(add::add_stat_aud)
    );
}
