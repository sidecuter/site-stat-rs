use serde::Deserialize;
use utoipa::ToSchema;
use crate::schemas::validators::{ApiKey, Page, Size};

#[derive(Deserialize, Clone, ToSchema)]
pub struct Filter{
    #[allow(dead_code)]
    #[schema(pattern = r"^[0-9a-f]{64}$", example = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")]
    pub api_key: ApiKey,
    pub user_id: Option<uuid::Uuid>,
    #[schema(example = 1, minimum = 1)]
    #[serde(default)]
    pub page: Page,
    #[schema(example = 50, maximum = 100)]
    #[serde(default)]
    pub size: Size
}
