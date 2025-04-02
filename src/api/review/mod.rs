use actix_web::web;

pub mod add;
pub mod get;
pub mod image;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/review")
            .service(get::get_reviews)
            .service(add::add_review)
            .service(image::get_image)
    );
}