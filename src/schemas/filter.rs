use serde::Deserialize;
use regex::Regex;
use utoipa::ToSchema;
use crate::schemas::api_key::ApiKey;

#[derive(Deserialize, Clone, ToSchema)]
pub struct Filter{
    #[schema(pattern = r"^[0-9a-f]{64}$", example = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")]
    api_key: ApiKey,
    user_id: Option<uuid::Uuid>,
    #[serde(default = 1)]
    page: u32,
    #[serde(default = 50)]
    size: u32
}
