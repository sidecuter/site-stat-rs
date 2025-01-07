use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct Status {
    pub status: String
}

impl Default for Status {
    fn default() -> Self {
        Self{status: "OK".to_string()}
    }
}

impl Responder for Status {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
