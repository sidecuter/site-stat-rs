use actix_web::{body::BoxBody, Responder};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResp {
    pub(crate) login: String,
    pub(crate) is_active: bool,
}

impl Responder for UserResp {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
