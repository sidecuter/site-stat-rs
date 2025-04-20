use crate::impl_responder;
use serde::Serialize;
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(test, derive(serde::Deserialize))]
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
