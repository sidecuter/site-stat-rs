mod db;
mod fetchers;
mod get;
mod helpers;
mod redoc;
mod review;
mod stat;

use self::helpers::prepare_database;
use migration::{Migrator, MigratorTrait};
use rstest::fixture;
use sea_orm::{Database, DatabaseConnection};

#[fixture]
async fn prepare_connection() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let pool = Database::connect("sqlite::memory:").await;
    let pool = pool?;
    Migrator::up(&pool, None).await?;
    prepare_database(&pool).await?;
    Ok(pool)
}
