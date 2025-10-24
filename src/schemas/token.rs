use crate::impl_responder;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub(crate) access_token: String,
    pub(crate) token_type: String,
}

impl_responder!(TokenResponse);
