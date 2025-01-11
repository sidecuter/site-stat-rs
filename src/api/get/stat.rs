use actix_web::{get, web};
use chrono::{NaiveDateTime, NaiveTime};
use itertools::Itertools;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
use entity::prelude::{ChangePlan, SelectAud, SiteStat, StartWay, UserId};
use crate::schemas::filter::{FilterQuery, Target};
use crate::schemas::{Statistics, Status};
use crate::errors::Result as ApiResult;

#[utoipa::path(
    get,
    path = "/api/get/stat",
    request_body = FilterQuery,
    responses(
        (
            status = 200, description = "Statistics", body = Statistics
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        )
    ),
    tag = "Get"
)]
#[get("/stat")]
async fn get_stat(
    data: web::Query<FilterQuery>,
    db: web::Data<DatabaseConnection>
) -> ApiResult<Statistics> {
    let period = match (data.start_date, data.end_date) {
        (Some(st_date), Some(end_date)) => {
            let st_datetime = NaiveDateTime::new(st_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            let end_datetime = NaiveDateTime::new(end_date, NaiveTime::from_hms_opt(23, 59, 59).unwrap());
            Some((st_datetime, end_datetime))
        }
        (Some(st_date), None) => {
            let st_datetime = NaiveDateTime::new(st_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            let end_datetime = NaiveDateTime::new(st_date, NaiveTime::from_hms_opt(23, 59, 59).unwrap());
            Some((st_datetime, end_datetime))
        },
        (_, _) => None
    };
    let users: Vec<_> = match (&data.target, period) {
        (Target::Site, Some(period)) => {
            let sites = SiteStat::find()
                .filter(entity::site_stat::Column::VisitDate.gte(period.0))
                .filter(entity::site_stat::Column::VisitDate.lte(period.1))
                .all(db.get_ref()).await?;
            sites.load_one(UserId, db.as_ref()).await?
        },
        (Target::Site, None) => {
            let sites = SiteStat::find().all(db.get_ref()).await?;
            sites.load_one(UserId, db.as_ref()).await?
        },
        (Target::Auds, Some(period)) => {
            let auds = SelectAud::find()
                .filter(entity::select_aud::Column::VisitDate.gte(period.0))
                .filter(entity::select_aud::Column::VisitDate.lte(period.1))
                .all(db.get_ref()).await?;
            auds.load_one(UserId, db.as_ref()).await?
        },
        (Target::Auds, None) => {
            let auds = SelectAud::find().all(db.get_ref()).await?;
            auds.load_one(UserId, db.as_ref()).await?
        },
        (Target::Ways, Some(period)) => {
            let ways = StartWay::find()
                .filter(entity::select_aud::Column::VisitDate.gte(period.0))
                .filter(entity::select_aud::Column::VisitDate.lte(period.1))
                .all(db.get_ref()).await?;
            ways.load_one(UserId, db.as_ref()).await?
        },
        (Target::Ways, None) => {
            let ways = StartWay::find().all(db.get_ref()).await?;
            ways.load_one(UserId, db.as_ref()).await?
        },
        (Target::Plans, Some(period)) => {
            let plans = ChangePlan::find()
                .filter(entity::select_aud::Column::VisitDate.gte(period.0))
                .filter(entity::select_aud::Column::VisitDate.lte(period.1))
                .all(db.get_ref()).await?;
            plans.load_one(UserId, db.as_ref()).await?
        },
        (Target::Plans, None) => {
            let plans = ChangePlan::find().all(db.get_ref()).await?;
            plans.load_one(UserId, db.as_ref()).await?
        }
    };
    let mut users: Vec<_> = users.into_iter().map(|v| v.unwrap()).collect();
    let all = users.iter().count();
    users.sort_by(|a, b| a.user_id.cmp(&b.user_id));
    let no_dup: Vec<_> = users.iter().cloned().dedup().collect();
    let unique = if let Some(period) = period {
        no_dup.iter().filter(|&a| (a.creation_date >= period.0) && (a.creation_date <= period.1)).count()
    } else {
        no_dup.iter().count()
    };
    let count = all - unique;
    Ok(Statistics {unique, count, all, period})
}