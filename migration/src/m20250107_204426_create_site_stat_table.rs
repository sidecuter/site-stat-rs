use crate::m20220101_000001_create_table::UserId;
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
                    .table(SiteStat::Table)
                    .if_not_exists()
                    .col(pk_auto(SiteStat::Id))
                    .col(ColumnDef::new(SiteStat::UserId).uuid().not_null())
                    .col(ColumnDef::new(SiteStat::VisitDate).date_time().not_null())
                    .col(ColumnDef::new(SiteStat::Endpoint).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(SiteStat::Table, SiteStat::UserId)
                            .to(UserId::Table, UserId::UserId)
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
            .drop_table(Table::drop().table(SiteStat::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SiteStat {
    Table,
    Id,
    UserId,
    Endpoint,
    VisitDate,
}
