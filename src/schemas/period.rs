use crate::schemas::FilterQuery;
use chrono::{NaiveDateTime, NaiveTime};
use serde::Serialize;
use actix_web::web::Query;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug, Clone)]
#[cfg_attr(test, derive(serde::Deserialize))]
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
