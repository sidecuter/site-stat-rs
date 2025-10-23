pub mod index;
pub mod playground;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/graphql")
            .service(index::index)
            .service(playground::graphql_playground),
    );
}
