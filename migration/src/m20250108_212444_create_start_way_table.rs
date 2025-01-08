use sea_orm_migration::{prelude::*, schema::*};
use crate::m20220101_000001_create_table::UserId;
use crate::m20250108_120158_create_auds_table::Aud;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(StartWay::Table)
                    .col(pk_auto(StartWay::Id))
                    .col(uuid(StartWay::UserId))
                    .col(string(StartWay::StartId).not_null())
                    .col(string(StartWay::EndId).not_null())
                    .col(date_time(StartWay::VisitDate).not_null())
                    .col(boolean(StartWay::Success).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(StartWay::Table, StartWay::UserId)
                            .to(UserId::Table, UserId::UserId)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StartWay::Table, StartWay::StartId)
                            .to(Aud::Table, Aud::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StartWay::Table, StartWay::EndId)
                            .to(Aud::Table, Aud::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned()
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(StartWay::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StartWay {
    Table,
    Id,
    UserId,
    StartId,
    EndId,
    VisitDate,
    Success
}
