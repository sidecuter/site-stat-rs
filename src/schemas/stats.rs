use crate::impl_responder;
use crate::schemas::Period;
use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug, Clone)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Statistics {
    pub unique: u64,
    pub count: u64,
    pub all: u64,
    pub period: Period,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            unique: 0,
            count: 10,
            all: 100,
            period: Period(None),
        }
    }
}

impl_responder!(Statistics);
