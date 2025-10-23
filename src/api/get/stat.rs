use crate::auth::IsCapable;
use crate::errors::{ApiError, ApiResult};
use crate::schemas::{goals, rights, FilterQuery, Period, Query, Statistics, Status};
use actix_web::{get, web};
use sea_orm::DatabaseConnection;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/get/stat",
    params(
        ("Authorization" = inline(String), Header, minimum = 7, example = "Bearer <token>"),
        ("target" = inline(crate::schemas::Target), Query),
        ("start_date" = inline(Option<chrono::NaiveDate>), Query, example = "2025-01-11"),
        ("end_date" = inline(Option<chrono::NaiveDate>), Query, example = "2025-01-12"),
    ),
    responses(
        (
            status = 200, description = "Statistics", body = Statistics,
            example = json!(Statistics::default())
        ),
        (
            status = 401, description = "User is inactive or not present", body = Status,
            example = json!(Status{status: "User is inactive or not present".to_string()})
        ),
        (
            status = 422, description = "Validation failed", body = Status,
            example = json!(Status{status: "parsing error...".to_string()})
        ),
        (
            status = 500, description = "Internal errors", body = Status,
            example = json!(Status{status: "internal error".to_string()})
        )
    ),
    security(
        ("oauth2_bearer" = ["view::stats"])
    ),
    tag = "Get"
)]
#[get("/stat")]
async fn get_stat(
    data: web::Query<FilterQuery>,
    db: web::Data<DatabaseConnection>,
    _is_capable: IsCapable<rights::View, goals::Stats>,
) -> ApiResult<Statistics> {
    match data.validate() {
        Ok(()) => Ok(()),
        Err(e) => Err(ApiError::UnprocessableData(e.to_string())),
    }?;
    let period: Period = (&data).into();
    let query: Query = (&data.target).into();
    Ok(query.count(db.get_ref(), &period).await?)
}
