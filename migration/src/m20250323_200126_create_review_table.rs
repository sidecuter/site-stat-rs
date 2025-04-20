use crate::m20220101_000001_create_table::UserId;
use crate::m20250323_195737_create_problem_table::Problem;
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
                    .table(Review::Table)
                    .if_not_exists()
                    .col(pk_auto(Review::Id))
                    .col(uuid(Review::UserId))
                    .col(date_time(Review::CreationDate))
                    .col(string(Review::Text))
                    .col(ColumnDef::new(Review::ImageName).string().null())
                    .col(string(Review::ProblemId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Review::Table, Review::UserId)
                            .to(UserId::Table, UserId::UserId)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Review::Table, Review::ProblemId)
                            .to(Problem::Table, Problem::Id)
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
            .drop_table(Table::drop().table(Review::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Review {
    Table,
    Id,
    UserId,
    ProblemId,
    Text,
    ImageName,
    CreationDate,
}
