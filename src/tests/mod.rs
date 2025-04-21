mod fetchers;
mod get;
mod helpers;
mod redoc;
mod review;
mod stat;

use self::helpers::prepare_database;
use actix_web::web::Data;
use migration::{Migrator, MigratorTrait};
use rstest::fixture;
use sea_orm::{Database, DatabaseConnection, DbBackend, MockDatabase};

#[fixture]
async fn prepare_connection() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let pool = Database::connect("sqlite::memory:").await;
    let pool = pool?;
    Migrator::up(&pool, None).await?;
    prepare_database(&pool).await?;
    Ok(pool)
}

fn get_db() -> Data<DatabaseConnection> {
    Data::new(MockDatabase::new(DbBackend::Sqlite).into_connection())
}
