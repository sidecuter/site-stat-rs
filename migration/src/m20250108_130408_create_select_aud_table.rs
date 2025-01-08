use sea_orm_migration::{prelude::*, schema::*};
use crate::m20220101_000001_create_table::UserId;
use crate::m20250108_120158_create_auds_table::Auds;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(SelectAud::Table)
                    .if_not_exists()
                    .col(pk_auto(SelectAud::Id))
                    .col(string(SelectAud::UserId).not_null())
                    .col(date_time(SelectAud::VisitData))
                    .col(string(SelectAud::AuditoryId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(SelectAud::Table, SelectAud::AuditoryId)
                            .to(Auds::Table, Auds::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SelectAud::Table, SelectAud::UserId)
                            .to(UserId::Table, UserId::UserId)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SelectAud::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SelectAud {
    Table,
    Id,
    UserId,
    VisitData,
    AuditoryId
}
