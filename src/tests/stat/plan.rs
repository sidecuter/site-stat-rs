use crate::api::stat::plan::stat_plan;
use crate::schemas::ChangePlanIn;
use crate::tests::db::{get_db, FillDb};
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::{DbBackend, MockDatabase};

#[rstest]
#[actix_web::test]
async fn test_200_stat_plan() {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .add_user_id()
            .add_plan()
            .add_change_plan()
            .add_exec_row()
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_plan)).await;
    let payload = ChangePlanIn {
        user_id: Default::default(),
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
#[actix_web::test]
async fn test_404_stat_plan_user() {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .add_empty_row()
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_plan)).await;
    let payload = ChangePlanIn {
        user_id: Default::default(),
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
#[actix_web::test]
async fn test_404_stat_plan_plan() {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .add_user_id()
            .add_empty_row()
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_plan)).await;
    let payload = ChangePlanIn {
        user_id: Default::default(),
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
#[actix_web::test]
async fn test_429_stat_plan_endpoint() {
    let app = test::init_service(App::new().app_data(get_db()).service(stat_plan)).await;
    let payload = ChangePlanIn {
        user_id: Default::default(),
        plan_id: "A-".into(),
    };
    let req = test::TestRequest::put()
        .uri("/change-plan")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 422);
}
