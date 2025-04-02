use actix_web::{get, web, middleware::from_fn};
use sea_orm::DatabaseConnection;
use validator::Validate;
use crate::schemas::{Filter, Pagination, SelectAuditoryOut, Status};
use crate::middleware::api_key_middleware;
use crate::errors::{ApiError, ApiResult};
use crate::traits::Paginate;

#[utoipa::path(
    get,
    path = "/api/get/auds",
    params(
        ("Api-Key" = inline(String), Header, minimum = 64, maximum = 64, example = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
        ("user_id" = inline(Option<uuid::Uuid>), Query, example = "84f332ed-fedc-48f6-9119-c6833932646f"),
        ("page" = inline(Option<u64>), Query, minimum = 1, example = "1"),
        ("size" = inline(Option<u64>), Query, maximum = 100, example = "50"),
    ),
    responses(
        (
            status = 200, description = "Paginated output for selected auditories", body = Pagination<SelectAuditoryOut>
        ),
        (
            status = 403, description = "ApiKey validation error", body = Status,
            example = json!(Status{status: "Specified api_key is not present in app".to_string()})
        ),
        (
            status = 422, description = "Validation failed", body = Status,
            example = json!(Status{status: "parsing error...".to_string()})
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        )
    ),
    tag = "Get"
)]
#[get("/auds", wrap = "from_fn(api_key_middleware)")]
async fn get_auds(
    data: web::Query<Filter>,
    db: web::Data<DatabaseConnection>,
) -> ApiResult<Pagination<SelectAuditoryOut>> {
    match data.validate() {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::UnprocessableData(e.to_string()))
    }?;
    Ok(SelectAuditoryOut::pagination(db.get_ref(), &data).await?)
}
