use crate::api::stat::plan::stat_plan;
use crate::schemas::ChangePlanIn;
use crate::tests::db::{add_change_plan, add_empty_row, add_exec_row, add_plan, add_user_id, get_db};
use actix_web::web::Data;
use actix_web::{test, App};
use sea_orm::{DbBackend, MockDatabase};
use rstest::*;

#[rstest]
#[actix_web::test]
async fn test_200_stat_plan() {
    let db = Data::new(
        add_exec_row(add_change_plan(add_plan(add_user_id(MockDatabase::new(DbBackend::Sqlite)))))
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
    let db = Data::new(add_empty_row(MockDatabase::new(DbBackend::Sqlite)).into_connection());
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
        add_empty_row(add_user_id(MockDatabase::new(DbBackend::Sqlite))).into_connection(),
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
