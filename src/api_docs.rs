use actix_web::{get, HttpResponse, Responder};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth routes
        crate::api::get::user_id::get_user_id
    ),
    components (
        schemas (
            // General schemas
            crate::schemas::status::Status,
            // Get schemas
            crate::schemas::user_id::UserId
        )
    )
)]
pub struct ApiDoc;

/// Return a json OpenAPI document
#[get("/openapi.json")]
pub async fn openapi_json() -> impl Responder {
    let openapi = ApiDoc::openapi();
    HttpResponse::Ok().json(openapi)
}