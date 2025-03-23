pub mod add;
pub mod get;

use actix_web::web;
use crate::app_state::AppState;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/review")
            .service(add::add_review)
            .service(get::get_reviews)
            .service(
                actix_files::Files::new(
                    "/image",
                    AppState::new().files_path.clone()
                ))
    );
}
