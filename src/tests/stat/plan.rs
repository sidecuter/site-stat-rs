use crate::api::stat::plan::stat_plan;
use crate::entity::{change_plan, plan, user_id};
use crate::schemas::ChangePlanIn;
use crate::tests::get_db;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::{DbBackend, MockDatabase, MockExecResult, MockRow};
use std::str::FromStr;

#[rstest]
#[tokio::test]
async fn test_200_stat_plan() {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .append_query_results([[user_id::Model {
                user_id: uuid::Uuid::from_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
                creation_date: chrono::Utc::now().naive_utc(),
            }]])
            .append_query_results([[plan::Model {
                id: "A-0".to_string(),
            }]])
            .append_query_results([[change_plan::Model {
                id: 0,
                user_id: uuid::Uuid::from_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
                visit_date: chrono::Utc::now().naive_utc(),
                plan_id: "A-0".to_string(),
            }]])
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_plan)).await;
    let payload = ChangePlanIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
        plan_id: "A-0".to_string(),
    };
    let req = test::TestRequest::put()
        .uri("/change-plan")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[rstest]
#[tokio::test]
async fn test_404_stat_plan_user() {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .append_query_results::<MockRow, Vec<_>, Vec<Vec<_>>>(vec![vec![]])
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_plan)).await;
    let payload = ChangePlanIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1").unwrap(),
        plan_id: "A-0".to_string(),
    };
    let req = test::TestRequest::put()
        .uri("/change-plan")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[rstest]
#[tokio::test]
async fn test_404_stat_plan_plan() {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .append_query_results([[user_id::Model {
                user_id: uuid::Uuid::from_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1").unwrap(),
                creation_date: chrono::Utc::now().naive_utc(),
            }]])
            .append_query_results::<MockRow, Vec<_>, Vec<Vec<_>>>(vec![vec![]])
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_plan)).await;
    let payload = ChangePlanIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1").unwrap(),
        plan_id: "A-8".to_string(),
    };
    let req = test::TestRequest::put()
        .uri("/change-plan")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[rstest]
#[tokio::test]
async fn test_429_stat_plan_endpoint() {
    let app = test::init_service(App::new().app_data(get_db()).service(stat_plan)).await;
    let payload = ChangePlanIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1").unwrap(),
        plan_id: "A-".into(),
    };
    let req = test::TestRequest::put()
        .uri("/change-plan")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 422);
}
