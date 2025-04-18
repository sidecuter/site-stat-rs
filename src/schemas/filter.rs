use std::fmt::{Display, Formatter};
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

#[derive(Deserialize, Clone, Debug, Default)]
pub enum Location {
    #[default]
    #[serde(rename = "campus_BS")]
    CampusBS,
    #[serde(rename = "campus_AV")]
    CampusAV,
    #[serde(rename = "campus_PR")]
    CampusPR,
    #[serde(rename = "campus_PK")]
    CampusPK,
    #[serde(rename = "campus_M")]
    CampusM,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Location::CampusBS => "BS",
            Location::CampusAV => "AV",
            Location::CampusPR => "PR",
            Location::CampusPK => "PK",
            Location::CampusM => "M"
        })
    }
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct FilterRoute {
    pub from_p: String,
    pub to_p: String,
    pub loc: Location
}
