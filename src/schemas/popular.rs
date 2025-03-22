use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Clone, ToSchema)]
pub struct Popular(pub Vec<String>);

impl Responder for Popular {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
