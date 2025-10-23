use crate::errors::ApiResult;
use actix_web::{get, HttpResponse};
use seaography::async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

#[utoipa::path(
    get,
    path = "/api/graphql/",
)]
#[get("/")]
async fn graphql_playground() -> ApiResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/api/graphql/"))))
}
