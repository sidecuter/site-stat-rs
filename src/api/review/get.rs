use crate::auth::IsCapable;
use crate::errors::{ApiError, ApiResult};
use crate::schemas::{goals, rights, Filter, Pagination, ReviewOut, Status};
use crate::traits::Paginate;
use actix_web::{get, web};
use sea_orm::DatabaseConnection;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/review/get",
    params(
        ("Authorization" = inline(String), Header, minimum = 7, example = "Bearer <token>"),
        ("user_id" = inline(Option<uuid::Uuid>), Query, example = "84f332ed-fedc-48f6-9119-c6833932646f"),
        ("page" = inline(Option<u64>), Query, minimum = 1, example = "1"),
        ("size" = inline(Option<u64>), Query, maximum = 100, example = "50"),
    ),
    responses(
        (
            status = 200, description = "Paginated output for selected reviews", body = Pagination<ReviewOut>
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
    tag = "Review"
)]
#[get("/get")]
async fn get_reviews(
    data: web::Query<Filter>,
    db: web::Data<DatabaseConnection>,
    _is_capable: IsCapable<rights::View, goals::Reviews>,
) -> ApiResult<Pagination<ReviewOut>> {
    match data.validate() {
        Ok(()) => Ok(()),
        Err(e) => Err(ApiError::UnprocessableData(e.to_string())),
    }?;
    Ok(ReviewOut::pagination(db.get_ref(), &data).await?)
}
