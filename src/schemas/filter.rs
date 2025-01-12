use crate::schemas::validators::{ApiKey, Page, Size};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Clone, ToSchema)]
pub struct Filter {
    #[allow(dead_code)]
    #[schema(
        pattern = r"^[0-9a-f]{64}$",
        example = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    )]
    pub api_key: ApiKey,
    pub user_id: Option<uuid::Uuid>,
    #[schema(example = 1, minimum = 1)]
    #[serde(default)]
    pub page: Page,
    #[schema(example = 50, maximum = 100)]
    #[serde(default)]
    pub size: Size,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub enum Target {
    Site,
    Auds,
    Ways,
    Plans,
}

#[derive(Deserialize, Clone, Debug, ToSchema)]
#[serde(tag = "target")]
pub struct FilterQuery {
    #[allow(dead_code)]
    #[schema(
        pattern = r"^[0-9a-f]{64}$",
        example = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    )]
    pub api_key: ApiKey,
    pub target: Target,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}
