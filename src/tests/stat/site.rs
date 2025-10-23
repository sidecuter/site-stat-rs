use crate::api::stat::site::stat_site;
use crate::schemas::SiteStatisticsIn;
use crate::tests::fixtures::prepare_connection;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::DatabaseConnection;
use utoipa::gen::serde_json::json;

#[rstest]
#[actix_web::test]
async fn test_200_stat_site(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(App::new().app_data(db).service(stat_site)).await;
    let payload = SiteStatisticsIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
        endpoint: None,
    };
    let req = test::TestRequest::put()
        .uri("/site")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[rstest]
#[actix_web::test]
async fn test_404_stat_site_user(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(App::new().app_data(db).service(stat_site)).await;
    let payload = SiteStatisticsIn {
        user_id: Default::default(),
        endpoint: None,
    };
    let req = test::TestRequest::put()
        .uri("/site")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[rstest]
#[actix_web::test]
async fn test_400_stat_site(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(App::new().app_data(db).service(stat_site)).await;
    let req = test::TestRequest::put()
        .uri("/site")
        .set_json(json!({
            "user_id": "11e1a4b8-7fa7-4501-9faa-541a5e0ff1e"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
