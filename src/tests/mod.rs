use std::sync::Mutex;
use once_cell::sync::Lazy;
use rstest::fixture;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use entity::{
    user_id::ActiveModel as UserId,
    site_stat::ActiveModel as SiteStat,
    select_aud::ActiveModel as SelectAuditory,
    start_way::ActiveModel as StartWay,
    change_plan::ActiveModel as ChangePlan
};
use migration::{Migrator, MigratorTrait};

pub static DATABASE_CONN: Lazy<Mutex<Option<DatabaseConnection>>> = Lazy::new(|| {
    Mutex::new(None)
});

async fn prepare_database(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    UserId {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        creation_date: Set(chrono::Utc::now().naive_utc())
    }.insert(db).await?;
    SiteStat {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        endpoint: Set(None),
        ..Default::default()
    }.insert(db).await?;
    SelectAuditory {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        auditory_id: Set("a-100".into()),
        success: Set(true),
        ..Default::default()
    }.insert(db).await?;
    StartWay {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        start_id: Set("a-100".into()),
        end_id: Set("a-101".into()),
        ..Default::default()
    }.insert(db).await?;
    ChangePlan {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        plan_id: Set("A-0".into()),
        ..Default::default()
    }.insert(db).await?;
    Ok(())
}

#[fixture]
async fn prepare_connection() -> Result<(), Box<dyn std::error::Error>> {
    let mut db_lock = DATABASE_CONN.lock().unwrap();
    if db_lock.is_none() {
        let pool = Database::connect("sqlite::memory:").await;
        println!("{}", pool.is_err());
        let pool = pool?;
        Migrator::up(&pool, None).await?;
        prepare_database(&pool).await?;
        *db_lock = Some(pool);
    }
    Ok(())
}

#[fixture]
#[once]
fn set_env() {
    std::env::set_var("RATE_LIMIT_INTERVAL", "1");
    std::env::set_var("RATE_LIMIT_MAX_REQUEST", "1");
}
