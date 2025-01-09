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
                    .table(Plan::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Plan::Id)
                            .string()
                            .not_null()
                            .primary_key()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Plan::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Plan {
    Table,
    Id,
}
