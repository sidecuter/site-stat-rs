use crate::errors::ApiResult;
use crate::middleware::api_key_middleware;
use crate::schemas::{Filter, Pagination, StartWayOut, Status};
use crate::traits::Paginate;
use actix_web::{get, middleware::from_fn, web};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    get,
    path = "/api/get/ways",
    params(
        ("api_key" = inline(crate::schemas::validators::ApiKey), Query),
        ("user_id" = inline(Option<uuid::Uuid>), Query, example = "84f332ed-fedc-48f6-9119-c6833932646f"),
        ("page" = inline(Option<crate::schemas::validators::Page>), Query, minimum = 1, example = "1"),
        ("size" = inline(Option<crate::schemas::validators::Page>), Query, maximum = 100, example = "50"),
    ),
    responses(
        (
            status = 200, description = "Paginated output for started ways", body = Pagination<StartWayOut>
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
#[get("/ways", wrap = "from_fn(api_key_middleware)")]
async fn get_ways(
    data: web::Query<Filter>,
    db: web::Data<DatabaseConnection>,
) -> ApiResult<Pagination<StartWayOut>> {
    Ok(StartWayOut::pagination(db.get_ref(), &data).await?)
}
