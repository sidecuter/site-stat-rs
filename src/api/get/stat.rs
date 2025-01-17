use crate::errors::Result as ApiResult;
use crate::middleware::api_key_middleware;
use crate::schemas::{FilterQuery, Period, Query, Statistics, Status};
use actix_web::{get, middleware::from_fn, web};
use itertools::Itertools;
use sea_orm::DatabaseConnection;

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
#[get("/stat", wrap = "from_fn(api_key_middleware)")]
async fn get_stat(
    data: web::Query<FilterQuery>,
    db: web::Data<DatabaseConnection>,
) -> ApiResult<Statistics> {
    let period: Period = (&data).into();
    let query: Query = (&data.target).into();
    let mut users = query.attach_period(&period).get_users(db.get_ref()).await?;
    let all = users.iter().count();
    users.sort_by(|a, b| a.user_id.cmp(&b.user_id));
    let no_dup: Vec<_> = users.iter().cloned().dedup().collect();
    let unique = if let Period(Some(period)) = period {
        no_dup
            .iter()
            .filter(|&a| (a.creation_date >= period.0) && (a.creation_date <= period.1))
            .count()
    } else {
        no_dup.iter().count()
    };
    let count = all - unique;
    Ok(Statistics {
        unique,
        count,
        all,
        period,
    })
}
