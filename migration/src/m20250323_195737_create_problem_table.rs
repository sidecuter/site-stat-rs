use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Problem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Problem::Id).string().not_null().primary_key())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Problem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Problem {
    Table,
    Id
}
