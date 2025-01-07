use actix_web::{get, HttpResponse, Responder};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth routes
        crate::api::get::user_id::get_user_id,
        crate::api::stat::site::stat_site
    ),
    components (
        schemas (
            // General schemas
            crate::schemas::status::Status,
            // Get schemas
            crate::schemas::user_id::UserId,
            //Validation schemas
            crate::schemas::site_stat::SiteStatisticsIn
        )
    ),
    tags (
        (name = "Get", description = "Getters for content"),
        (name = "Stat", description = "Statistics insertion endpoints"),
    ),
)]
pub struct ApiDoc;

/// Return a json OpenAPI document
#[get("/openapi.json")]
pub async fn openapi_json() -> impl Responder {
    let openapi = ApiDoc::openapi();
    HttpResponse::Ok().json(openapi)
}