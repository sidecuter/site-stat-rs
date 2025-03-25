use actix_web::{get, middleware::from_fn, web};
use sea_orm::DatabaseConnection;
use validator::Validate;
use crate::schemas::{FilterQuery, Period, Query, Statistics, Status};
use crate::middleware::api_key_middleware;
use crate::errors::{ApiError, ApiResult};

#[utoipa::path(
    get,
    path = "/api/get/stat",
    params(
        ("api_key" = inline(String), Query, minimum = 64, maximum = 64, example = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
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
    match data.validate() {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::UnprocessableData(e.to_string()))
    }?;
    let period: Period = (&data).into();
    let query: Query = (&data.target).into();
    Ok(query.count(db.get_ref(), &period).await?)
}
