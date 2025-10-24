use actix_web::{post, web};
use async_graphql::dynamic::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use seaography::async_graphql;

#[utoipa::path(
    post,
    path = "/api/graphql",
    tag = "GraphQL"
)]
#[post("/graphql")]
async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
