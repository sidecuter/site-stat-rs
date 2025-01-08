use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::EntityTrait;
use crate::data::AUDS;
use entity::{aud::{ActiveModel}, prelude::Aud};
use crate::sea_orm::ActiveValue::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let auds: Vec<_> = AUDS.iter().map(|aud| ActiveModel {id: Set(aud.to_string())}).collect();
        // Replace the sample below with your own migration scripts
        Aud::insert_many(auds).exec(db).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        Aud::delete_many().exec(db).await.map(|_| ())
    }
}
