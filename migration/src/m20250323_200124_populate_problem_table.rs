use crate::data::PROBLEMS;
use crate::m20250323_195737_create_problem_table::Problem;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // Replace the sample below with your own migration scripts
        let mut insert = Query::insert()
            .into_table(Problem::Table)
            .columns([Problem::Id])
            .to_owned();
        for problem in PROBLEMS {
            insert = insert.values_panic([problem.into()]).to_owned();
        }
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let delete = Query::delete().from_table(Problem::Table).to_owned();
        manager.exec_stmt(delete).await?;
        Ok(())
    }
}
