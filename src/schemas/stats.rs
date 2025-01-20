use crate::schemas::Period;
use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Statistics {
    pub unique: u64,
    pub count: u64,
    pub all: u64,
    pub period: Period,
}

impl Responder for Statistics {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            unique: 0,
            count: 10,
            all: 100,
            period: Period(None)
        }
    }
}
