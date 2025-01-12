use actix_web::{get, HttpResponse, Responder};
use utoipa::OpenApi;
use utoipauto::utoipauto;

#[utoipauto]
#[derive(OpenApi)]
#[openapi(
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
