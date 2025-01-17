use crate::data::AUDS;
use crate::m20250108_120158_create_auds_table::Aud;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let mut insert = Query::insert()
            .into_table(Aud::Table)
            .columns([Aud::Id])
            .to_owned();
        for aud in AUDS {
            insert = insert.values_panic([aud.into()]).to_owned();
        }
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let delete = Query::delete().from_table(Aud::Table).to_owned();
        manager.exec_stmt(delete).await?;
        Ok(())
    }
}
