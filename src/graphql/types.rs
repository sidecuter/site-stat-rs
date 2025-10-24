use chrono::NaiveDate;
use sea_orm::{FromQueryResult};
use seaography::{CustomInputType, CustomOutputType};

#[derive(Clone, CustomOutputType, FromQueryResult)]
pub struct Statistics {
    pub period_str: String,
    pub all_visits: i32,
    pub visitor_count: i32,
    pub unique_visits: i32,
}

#[derive(Clone, CustomInputType)]
pub struct FilterQuery {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}
