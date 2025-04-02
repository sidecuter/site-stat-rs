use actix_web::{get, HttpResponse, Responder};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Get routes
        crate::api::user_id::get::get_user_id,
        crate::api::site::get::get_sites,
        crate::api::aud::get::get_auds,
        crate::api::way::get::get_ways,
        crate::api::plan::get::get_plans,
        crate::api::stat::get::get_stat,
        crate::api::popular::get::get_popular,
        // Set routes
        crate::api::site::add::add_stat_site,
        crate::api::aud::add::add_stat_aud,
        crate::api::way::add::add_stat_way,
        crate::api::plan::add::add_stat_plan,
        // Review routes
        crate::api::review::add::add_review,
        crate::api::review::get::get_reviews,
        crate::api::review::image::get_image,
    ),
    components (
        schemas (
            crate::schemas::status::Status,
            crate::schemas::UserId,
            crate::schemas::SiteStatisticsIn,
            crate::schemas::SiteStatisticsOut,
            crate::schemas::SelectAuditoryIn,
            crate::schemas::SelectAuditoryOut,
            crate::schemas::StartWayIn,
            crate::schemas::StartWayOut,
            crate::schemas::ChangePlanIn,
            crate::schemas::ChangePlanOut,
            crate::schemas::Filter,
            crate::schemas::Target,
            crate::schemas::FilterQuery,
            crate::schemas::stats::Statistics,
            crate::schemas::review::ReviewFormIn,
            crate::schemas::review::ReviewOut,
            crate::schemas::popular::Popular,
            crate::schemas::period::Period,
        )
    ),
    tags (
        (name = "UserId", description = "Methods for user_id"),
        (name = "Site", description = "Methods for site visits stats"),
        (name = "Aud", description = "Methods for auditory select stats"),
        (name = "Way", description = "Methods for started ways stats"),
        (name = "Plan", description = "Methods for changed plans stats"),
        (name = "Stat", description = "Statistics methods"),
        (name = "Review", description = "Endpoint for reviews"),
        (name = "Popular", description = "Methods for popular auditories"),
    ),
)]
pub struct ApiDoc;

/// Return a json OpenAPI document
#[get("/openapi.json")]
pub async fn openapi_json() -> impl Responder {
    let openapi = ApiDoc::openapi();
    HttpResponse::Ok().json(openapi)
}
