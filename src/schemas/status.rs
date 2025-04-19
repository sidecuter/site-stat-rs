use serde::{Deserialize, Serialize};
use actix_web::body::BoxBody;
use actix_web::Responder;
use std::str::FromStr;
use utoipa::ToSchema;
use crate::impl_responder;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Eq, PartialEq)]
pub struct Status {
    #[schema(example = "OK")]
    pub status: String,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            status: "OK".to_string(),
        }
    }
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            status: s.to_string(),
        })
    }
}

impl_responder!(Status);
