use actix_web::{body::BoxBody, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub(crate) access_token: String,
    pub(crate) token_type: String,
}

impl Responder for TokenResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
