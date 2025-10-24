use crate::entity::{change_plan, select_aud, site_stat, start_way};
use crate::generate_statistics_query;
use crate::query_root::types::{FilterQuery, Statistics};
use async_graphql::Context;
use sea_orm::{DatabaseConnection, ExprTrait, FromQueryResult, Iden, IntoSimpleExpr};
use seaography::CustomFields;

struct DateFunc;

impl Iden for DateFunc {
    fn unquoted(&self) -> &str {
        "date"
    }
}

pub struct Operations;

#[CustomFields]
impl Operations {
    async fn site_stats(
        ctx: &Context<'_>,
        filter: Option<FilterQuery>,
    ) -> async_graphql::Result<Vec<Statistics>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let query = generate_statistics_query!(site_stat, filter);
        let statement = db.get_database_backend().build(&query);
        Ok(Statistics::find_by_statement(statement).all(db).await?)
    }

    async fn aud_stats(
        ctx: &Context<'_>,
        filter: Option<FilterQuery>,
    ) -> async_graphql::Result<Vec<Statistics>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let query = generate_statistics_query!(select_aud, filter);
        let statement = db.get_database_backend().build(&query);
        Ok(Statistics::find_by_statement(statement).all(db).await?)
    }

    async fn way_stats(
        ctx: &Context<'_>,
        filter: Option<FilterQuery>,
    ) -> async_graphql::Result<Vec<Statistics>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let query = generate_statistics_query!(start_way, filter);
        let statement = db.get_database_backend().build(&query);
        Ok(Statistics::find_by_statement(statement).all(db).await?)
    }

    async fn plan_stats(
        ctx: &Context<'_>,
        filter: Option<FilterQuery>,
    ) -> async_graphql::Result<Vec<Statistics>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let query = generate_statistics_query!(change_plan, filter);
        let statement = db.get_database_backend().build(&query);
        Ok(Statistics::find_by_statement(statement).all(db).await?)
    }
}
