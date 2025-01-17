use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::schemas::Period;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Statistics {
    pub unique: usize,
    pub count: usize,
    pub all: usize,
    pub period: Period,
}

impl Responder for Statistics {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
