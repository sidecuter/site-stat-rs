use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

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

impl Responder for Status {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
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
