use std::borrow::Cow;
use crate::schemas::validators::{page_default, size_default};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
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
#[validate(schema(function = "validate_filter_query"))]
pub struct FilterQuery {
    pub target: Target,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

fn validate_filter_query(schema: &FilterQuery) -> Result<(), ValidationError> {
    if let (Some(st_d), Some(end_d)) = (schema.start_date, schema.end_date) {
        if st_d > end_d {
            Err(
                ValidationError::new("invalid_date_range")
                    .with_message(Cow::from("start_date must be lesser than end_date"))
            )   
        } else { Ok(()) }
    } else { Ok(()) }
}
