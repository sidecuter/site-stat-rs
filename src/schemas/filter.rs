use crate::schemas::validators::{page_default, size_default};
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::NaiveDate;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct Filter {
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
    pub target: Target,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}
