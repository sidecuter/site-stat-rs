use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::EntityTrait;
use entity::plan::ActiveModel;
use entity::prelude::Plan;
use crate::data::PLANS;
use crate::sea_orm::ActiveValue::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        let plan: Vec<_> = PLANS.iter().map(|plan| ActiveModel {id: Set(plan.to_string())}).collect();
        Plan::insert_many(plan).exec(db).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let db = manager.get_connection();
        Plan::delete_many().exec(db).await.map(|_| ())
    }
}
