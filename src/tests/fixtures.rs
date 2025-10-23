use crate::auth::create_token;
use crate::tests::helpers::prepare_database;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

#[rstest::fixture]
#[once]
pub fn jwt_token() -> String {
    create_token(1, "524c9b6806b8f7ae95c56747d35432c7").unwrap()
}

#[rstest::fixture]
pub async fn prepare_connection() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let pool = Database::connect("sqlite::memory:").await?;
    Migrator::up(&pool, None).await?;
    prepare_database(&pool).await?;
    Ok(pool)
}
