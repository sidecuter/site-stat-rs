mod review;
mod helpers;
mod swagger;
mod user_id;
mod site;
mod gets;
mod aud;
mod way;
mod plan;
mod stat;
mod popular;

use migration::{Migrator, MigratorTrait};
use rstest::fixture;
use sea_orm::{Database, DatabaseConnection};
use self::helpers::prepare_database;

#[fixture]
async fn prepare_connection() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let pool = Database::connect("sqlite::memory:").await;
    let pool = pool?;
    Migrator::up(&pool, None).await?;
    prepare_database(&pool).await?;
    Ok(pool)
}
