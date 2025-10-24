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
        crate::api::get::popular::get_popular,
        crate::api::get::route::get_route,
        // Set routes
        crate::api::stat::site::stat_site,
        crate::api::stat::aud::stat_aud,
        crate::api::stat::way::stat_way,
        crate::api::stat::plan::stat_plan,
        // Review routes
        crate::api::review::add::add_review,
        crate::api::review::image::get_image,
        // Auth routes
        crate::api::auth::login::token,
        // Graphql routes
        crate::api::graphql::index::index,
        crate::api::graphql::playground::graphql_playground,
    ),
    components (
        schemas (
            crate::schemas::status::Status,
            crate::schemas::UserId,
            crate::schemas::SiteStatisticsIn,
            crate::schemas::SelectAuditoryIn,
            crate::schemas::StartWayIn,
            crate::schemas::ChangePlanIn,
            crate::schemas::Target,
            crate::schemas::FilterQuery,
            crate::schemas::review::ReviewFormIn,
            crate::schemas::popular::Popular,
            crate::schemas::period::Period,
            crate::schemas::graph::VertexType,
            crate::schemas::graph::Vertex,
            crate::schemas::graph::ShortestWay,
            crate::schemas::login_request::LoginRequest,
            crate::schemas::token::TokenResponse,
            crate::schemas::login_request::LoginRequest
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
