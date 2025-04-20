use crate::schemas::validators::{page_default, size_default};
use chrono::NaiveDate;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Clone, ToSchema, Validate)]
#[cfg_attr(test, derive(serde::Serialize))]
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

#[derive(Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(test, derive(serde::Serialize))]
pub enum Target {
    Site,
    Auds,
    Ways,
    Plans,
}

#[derive(Deserialize, Clone, Debug, ToSchema, Validate)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct FilterQuery {
    pub target: Target,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[cfg_attr(test, derive(serde::Serialize))]
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
        write!(
            f,
            "{}",
            match self {
                Location::CampusBS => "BS",
                Location::CampusAV => "AV",
                Location::CampusPR => "PR",
                Location::CampusPK => "PK",
                Location::CampusM => "M",
            }
        )
    }
}

#[cfg(test)]
impl From<&str> for Location {
    fn from(value: &str) -> Self {
        match value {
            "campus_BS" => Self::CampusBS,
            "campus_AV" => Self::CampusAV,
            "campus_PR" => Self::CampusPR,
            "campus_PK" => Self::CampusPK,
            "campus_M" => Self::CampusM,
            _ => panic!("No such campus"),
        }
    }
}

#[derive(Deserialize, Clone, Default, Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct FilterRoute {
    pub from_p: String,
    pub to_p: String,
    pub loc: Location,
}
