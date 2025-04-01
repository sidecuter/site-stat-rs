use crate::schemas::validators::{API_KEY_RE, page_default, size_default};
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::NaiveDate;
use utoipa::ToSchema;

#[derive(Deserialize, Clone, ToSchema, Validate)]
pub struct Filter {
    #[allow(dead_code)]
    #[schema(
        pattern = r"^[0-9a-f]{64}$",
        example = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    )]
    #[validate(length(equal = 64), regex(path = *API_KEY_RE))]
    pub api_key: String,
    pub user_id: Option<uuid::Uuid>,
    #[schema(example = 1, minimum = 1)]
    #[serde(default = "page_default")]
    #[validate(range(min = 1))]
    pub page: u64,
    #[schema(example = 50, maximum = 100)]
    #[serde(default = "size_default")]
    #[validate(range(max = 100))]
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    Site,
    Auds,
    Ways,
    Plans,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema, Validate)]
pub struct FilterQuery {
    #[allow(dead_code)]
    #[schema(
        pattern = r"^[0-9a-f]{64}$",
        example = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    )]
    #[validate(length(equal = 64), regex(path = *API_KEY_RE))]
    pub api_key: String,
    pub target: Target,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}
