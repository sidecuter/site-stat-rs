pub mod add;
pub mod image;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/review")
            .service(add::add_review)
            .service(image::get_image),
    );
}
