use crate::data::PLANS;
use sea_orm_migration::prelude::*;
use crate::m20250109_160455_create_plan_table::Plan;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let mut insert = Query::insert()
            .into_table(Plan::Table)
            .columns([Plan::Id])
            .to_owned();
        for plan in PLANS {
            insert = insert.values_panic([plan.into()]).to_owned();
        }
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let delete = Query::delete().from_table(Plan::Table).to_owned();
        manager.exec_stmt(delete).await?;
        Ok(())
    }
}
