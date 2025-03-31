mod get;
mod stat;
mod review;

use actix_web::{test, web, App};
use crate::entity::{
    change_plan::ActiveModel as ChangePlan, select_aud::ActiveModel as SelectAuditory,
    site_stat::ActiveModel as SiteStat, start_way::ActiveModel as StartWay,
    user_id::ActiveModel as UserId,
    review::ActiveModel as Review
};
use migration::{Migrator, MigratorTrait};
use rstest::{fixture, rstest};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;
use crate::api_docs;

async fn prepare_database(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    UserId {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        creation_date: Set(chrono::Utc::now().naive_utc()),
    }
        .insert(db)
        .await?;
    SiteStat {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        endpoint: Set(None),
        ..Default::default()
    }
        .insert(db)
        .await?;
    SelectAuditory {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        auditory_id: Set("a-100".into()),
        success: Set(true),
        ..Default::default()
    }
        .insert(db)
        .await?;
    StartWay {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        start_id: Set("a-100".into()),
        end_id: Set("a-101".into()),
        ..Default::default()
    }
        .insert(db)
        .await?;
    ChangePlan {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        plan_id: Set("A-0".into()),
        ..Default::default()
    }
        .insert(db)
        .await?;
    Review {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        creation_date: Set(chrono::Utc::now().naive_utc()),
        text: Set("Awesome review".to_owned()),
        problem_id: Set("way".to_owned()),
        ..Default::default()
    }
        .insert(db)
        .await?;
    Ok(())
}

#[fixture]
async fn prepare_connection() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let pool = Database::connect("sqlite::memory:").await;
    let pool = pool?;
    Migrator::up(&pool, None).await?;
    prepare_database(&pool).await?;
    Ok(pool)
}

#[rstest]
#[tokio::test]
async fn test_swagger() {
    let app = test::init_service(
        App::new()
            .service(
                // OpenAPI document
                web::scope("/docs").service(api_docs::openapi_json).service(
                    SwaggerUi::new("/swagger/{_:.*}").url("/docs/openapi.json", Default::default()),
                ),
            )
    ).await;
    let req = test::TestRequest::get().uri("/docs/openapi.json").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let req = test::TestRequest::get().uri("/docs/swagger/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200)
}
