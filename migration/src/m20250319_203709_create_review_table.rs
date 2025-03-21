use sea_orm_migration::{prelude::*, schema::*};
use crate::sea_orm::{EnumIter, Iterable};
use crate::m20220101_000001_create_table::UserId;

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
                    .col(ColumnDef::new(Review::ImageId).string().null())
                    .col(ColumnDef::new(Review::ImageExt).string().null())
                    .col(ColumnDef::new(Review::Problem)
                        .enumeration(
                            Alias::new("enum"),
                            Problem::iter()
                        )
                        .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Review::Table, Review::UserId)
                            .to(UserId::Table, UserId::UserId)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned()
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
    Problem,
    Text,
    ImageId,
    ImageExt,
    CreationDate
}

#[derive(Iden, EnumIter)]
pub enum Problem {
    #[iden = "other"]
    Other,
    #[iden = "way"]
    Way,
    #[iden = "work"]
    Work,
    #[iden = "plan"]
    Plan
}
