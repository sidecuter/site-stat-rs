use super::{queries, types};
use crate::entity::*;
use async_graphql::dynamic::*;
use async_graphql::extensions::{Extension, ExtensionContext, ExtensionFactory, NextParseQuery};
use async_graphql::parser::types::{ExecutableDocument, OperationType};
use async_graphql::{ServerError, ServerResult, Variables};
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext};
use std::sync::Arc;

lazy_static::lazy_static! { static ref CONTEXT : BuilderContext = BuilderContext :: default () ; }

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    builder = register_entity_modules(builder);
    seaography::register_custom_inputs!(builder, [types::FilterQuery]);
    seaography::register_custom_outputs!(builder, [types::Statistics]);
    seaography::register_custom_queries!(builder, [queries::Operations]);
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .extension(Readonly)
        .finish()
}

pub struct Readonly;

impl ExtensionFactory for Readonly {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(ReadonlyExtension)
    }
}

struct ReadonlyExtension;

#[async_trait::async_trait]
impl Extension for ReadonlyExtension {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        let document = next.run(ctx, query, variables).await?;
        let is_mutation = document
            .operations
            .iter()
            .any(|(_, operation)| matches!(operation.node.ty, OperationType::Mutation));
        if is_mutation {
            return Err(ServerError::new("Mutations is not allowed", None));
        }
        Ok(document)
    }
}
