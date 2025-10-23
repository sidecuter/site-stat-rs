use serde::Serialize;
use utoipa::openapi::security::{
    Flow, OAuth2, Password, Scopes, SecurityScheme,
};
use utoipa::{Modify, OpenApi};

#[derive(Debug, Serialize)]
struct Security;

impl Modify for Security {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "oauth2_bearer",
                SecurityScheme::OAuth2(OAuth2::new([Flow::Password(Password::new(
                    "http://localhost:8080/api/auth/token",
                    Scopes::new(),
                ))])),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&Security),
    paths(
        // Get routes
        crate::api::get::user_id::get_user_id,
        crate::api::get::sites::get_sites,
        crate::api::get::auds::get_auds,
        crate::api::get::ways::get_ways,
        crate::api::get::plans::get_plans,
        crate::api::get::popular::get_popular,
        crate::api::get::stat::get_stat,
        crate::api::get::route::get_route,
        // Set routes
        crate::api::stat::site::stat_site,
        crate::api::stat::aud::stat_aud,
        crate::api::stat::way::stat_way,
        crate::api::stat::plan::stat_plan,
        // Review routes
        crate::api::review::add::add_review,
        crate::api::review::get::get_reviews,
        crate::api::review::image::get_image,
        // Auth routes
        crate::api::auth::login::token,
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
            crate::schemas::graph::VertexType,
            crate::schemas::graph::Vertex,
            crate::schemas::graph::ShortestWay,
            crate::schemas::login_request::LoginRequest,
            crate::schemas::token::TokenResponse
        )
    ),
    tags (
        (name = "Get", description = "Getters for content"),
        (name = "Stat", description = "Statistics insertion endpoints"),
        (name = "Review", description = "Endpoint for reviews"),
        (name = "Auth", description = "Endpoints for authentication"),
    ),
)]
pub struct ApiDoc;
