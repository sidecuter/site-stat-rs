use actix_web::{get, HttpResponse, Responder};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Get routes
        crate::api::get::user_id::get_user_id,
        crate::api::get::sites::get_sites,
        crate::api::get::auds::get_auds,
        crate::api::get::ways::get_ways,
        // Set routes
        crate::api::stat::site::stat_site,
        crate::api::stat::aud::stat_aud,
        crate::api::stat::way::stat_way,
        crate::api::stat::plan::stat_plan
    ),
    components (
        schemas (
            // General schemas
            crate::schemas::status::Status,
            // Get schemas
            crate::schemas::UserId,
            crate::schemas::SiteStatisticsOut,
            crate::schemas::SelectAuditoryOut,
            crate::schemas::StartWayOut,
            crate::schemas::ChangePlanOut,
            //Validation schemas
            crate::schemas::SiteStatisticsIn,
            crate::schemas::SelectAuditoryIn,
            crate::schemas::StartWayIn,
            crate::schemas::ChangePlanIn,
            crate::schemas::Filter,
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