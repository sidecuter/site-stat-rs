use actix_web::{get, web, middleware::from_fn};
use sea_orm::DatabaseConnection;
use crate::errors::Result as ApiResult;
use crate::schemas::status::Status;
use crate::traits::{ConversionToPaginationTrait, Paginate};
use crate::middleware::api_key_middleware;
use crate::schemas::filter::Filter;
use crate::schemas::pagination::Pagination;
use crate::schemas::start_way::StartWayOut;

#[utoipa::path(
    get,
    path = "/api/get/ways",
    request_body = Filter,
    responses(
        (
            status = 200, description = "User id generated", body = Pagination<StartWayOut>
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        )
    ),
    tag = "Get"
)]
#[get("/ways", wrap="from_fn(api_key_middleware)")]
async fn get_ways(
    data: web::Query<Filter>,
    db: web::Data<DatabaseConnection>
) -> ApiResult<Pagination<StartWayOut>> {
    <Filter as Paginate<Pagination<StartWayOut>>>::pagination(&data, db.get_ref()).await.to_response()
}
