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
                    .table(UserId::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserId::UserId)
                            .primary_key()
                            .uuid()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(UserId::CreationDate)
                            .date_time()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(UserId::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserId {
    Table,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "creation_date")]
    CreationDate
}
