use crate::api::stat::way::stat_way;
use crate::schemas::StartWayIn;
use crate::tests::db::{add_aud, add_empty_row, add_exec_row, add_start_way, add_user_id, get_db};
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::{DbBackend, MockDatabase};

#[rstest]
#[actix_web::test]
async fn test_200_stat_way() {
    let db = Data::new(
        add_exec_row(add_start_way(add_aud(
            add_user_id(MockDatabase::new(DbBackend::Sqlite)),
            2,
        )))
        .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_way)).await;
    let payload = StartWayIn {
        user_id: Default::default(),
        start_id: "a-100".to_string(),
        end_id: "a-101".to_string(),
    };
    let req = test::TestRequest::put()
        .uri("/start-way")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[rstest]
#[actix_web::test]
async fn test_404_stat_way_user() {
    let db = Data::new(add_empty_row(MockDatabase::new(DbBackend::Sqlite)).into_connection());
    let app = test::init_service(App::new().app_data(db).service(stat_way)).await;
    let payload = StartWayIn {
        user_id: Default::default(),
        start_id: "a-100".to_string(),
        end_id: "a-101".to_string(),
    };
    let req = test::TestRequest::put()
        .uri("/start-way")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[rstest]
#[actix_web::test]
async fn test_404_stat_way_start() {
    let db = Data::new(
        add_empty_row(add_user_id(MockDatabase::new(DbBackend::Sqlite))).into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_way)).await;
    let payload = StartWayIn {
        user_id: Default::default(),
        start_id: "a-100".to_string(),
        end_id: "a-101".to_string(),
    };
    let req = test::TestRequest::put()
        .uri("/start-way")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[rstest]
#[actix_web::test]
async fn test_404_stat_way_end() {
    let db = Data::new(
        add_empty_row(add_aud(
            add_user_id(MockDatabase::new(DbBackend::Sqlite)),
            1,
        ))
        .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_way)).await;
    let payload = StartWayIn {
        user_id: Default::default(),
        start_id: "a-100".to_string(),
        end_id: "a-101".to_string(),
    };
    let req = test::TestRequest::put()
        .uri("/start-way")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[rstest]
#[actix_web::test]
async fn test_422_stat_way_endpoint() {
    let app = test::init_service(App::new().app_data(get_db()).service(stat_way)).await;
    let payload = StartWayIn {
        user_id: Default::default(),
        start_id: "a-101".into(),
        end_id: "a-".into(),
    };
    let req = test::TestRequest::put()
        .uri("/start-way")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 422);
}
