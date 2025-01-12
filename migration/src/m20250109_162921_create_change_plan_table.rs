use crate::m20220101_000001_create_table::UserId;
use crate::m20250109_160455_create_plan_table::Plan;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(ChangePlan::Table)
                    .if_not_exists()
                    .to_owned()
                    .col(pk_auto(ChangePlan::Id))
                    .col(uuid(ChangePlan::UserId))
                    .col(date_time(ChangePlan::VisitDate))
                    .col(string(ChangePlan::PlanId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ChangePlan::Table, ChangePlan::UserId)
                            .to(UserId::Table, UserId::UserId)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ChangePlan::Table, ChangePlan::PlanId)
                            .to(Plan::Table, Plan::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(ChangePlan::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ChangePlan {
    Table,
    Id,
    UserId,
    VisitDate,
    PlanId,
}
