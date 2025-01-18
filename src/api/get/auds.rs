use crate::errors::Result as ApiResult;
use crate::middleware::api_key_middleware;
use crate::schemas::{Filter, Pagination, SelectAuditoryOut, Status};
use crate::traits::Paginate;
use actix_web::{get, middleware::from_fn, web};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    get,
    path = "/api/get/auds",
    params(Filter),
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
    Ok(SelectAuditoryOut::pagination(db.get_ref(), &data).await?)
}
