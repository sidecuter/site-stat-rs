use crate::schemas::FilterQuery;
use actix_web::web::Query;
use chrono::{NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Period(pub Option<(NaiveDateTime, NaiveDateTime)>);

impl From<&Query<FilterQuery>> for Period {
    fn from(value: &Query<FilterQuery>) -> Self {
        match (value.start_date, value.end_date) {
            (Some(st_date), Some(end_date)) => {
                let st_datetime =
                    NaiveDateTime::new(st_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                let end_datetime =
                    NaiveDateTime::new(end_date, NaiveTime::from_hms_opt(23, 59, 59).unwrap());
                Self(Some((st_datetime, end_datetime)))
            }
            (Some(st_date), None) => {
                let st_datetime =
                    NaiveDateTime::new(st_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                let end_datetime =
                    NaiveDateTime::new(st_date, NaiveTime::from_hms_opt(23, 59, 59).unwrap());
                Self(Some((st_datetime, end_datetime)))
            }
            (_, _) => Self(None),
        }
    }
}
