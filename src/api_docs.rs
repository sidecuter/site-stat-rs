use actix_web::{get, HttpResponse, Responder};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Get routes
        crate::api::get::user_id::get_user_id,
        crate::api::get::sites::get_sites,
        crate::api::get::auds::get_auds,
        // Set routes
        crate::api::stat::site::stat_site,
        crate::api::stat::aud::stat_aud,
    ),
    components (
        schemas (
            // General schemas
            crate::schemas::status::Status,
            // Get schemas
            crate::schemas::user_id::UserId,
            crate::schemas::site_stat::SiteStatisticsOut,
            crate::schemas::select_aud::SelectAuditoryOut,
            //Validation schemas
            crate::schemas::site_stat::SiteStatisticsIn,
            crate::schemas::select_aud::SelectAuditoryIn,
            crate::schemas::filter::Filter,
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