use actix_web::{get, web, middleware::from_fn};
use sea_orm::DatabaseConnection;
use crate::errors::Result as ApiResult;
use crate::schemas::status::Status;
use crate::traits::{ConversionToPaginationTrait, Paginate};
use crate::middleware::api_key_middleware;
use crate::schemas::filter::Filter;
use crate::schemas::pagination::Pagination;
use crate::schemas::site_stat::SiteStatisticsOut;

#[utoipa::path(
    get,
    path = "/api/get/sites",
    request_body = Filter,
    responses(
        (
            status = 200, description = "User id generated", body = Pagination<SiteStatisticsOut>
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        )
    ),
    tag = "Get"
)]
#[get("/sites", wrap="from_fn(api_key_middleware)")]
async fn get_sites(
    data: web::Query<Filter>,
    db: web::Data<DatabaseConnection>
) -> ApiResult<Pagination<SiteStatisticsOut>> {
    data.pagination(db.get_ref()).await.to_response()
}
