use crate::api::stat::way::stat_way;
use crate::schemas::StartWayIn;
use crate::tests::fixtures::prepare_connection;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::DatabaseConnection;

#[rstest]
#[actix_web::test]
async fn test_200_stat_way(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(App::new().app_data(db).service(stat_way)).await;
    let payload = StartWayIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
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
async fn test_404_stat_way_user(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
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
async fn test_404_stat_way_start(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(App::new().app_data(db).service(stat_way)).await;
    let payload = StartWayIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
        start_id: "a-1000".to_string(),
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
async fn test_404_stat_way_end(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(App::new().app_data(db).service(stat_way)).await;
    let payload = StartWayIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
        start_id: "a-100".to_string(),
        end_id: "a-1010".to_string(),
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
async fn test_422_stat_way_endpoint(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(App::new().app_data(db).service(stat_way)).await;
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
