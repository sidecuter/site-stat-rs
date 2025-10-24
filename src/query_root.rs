mod queries;
pub mod types;

use crate::entity::*;
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext};

lazy_static::lazy_static! { static ref CONTEXT : BuilderContext = BuilderContext :: default () ; }

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    builder = register_entity_modules(builder);
    seaography::register_custom_inputs!(
        builder,
        [
            types::FilterQuery
        ]
    );
    seaography::register_custom_outputs!(builder, [types::Statistics]);
    seaography::register_custom_queries!(builder, [queries::Operations]);
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}
