use super::super::prepare_connection;
use crate::api::stat::site::stat_site;
use crate::schemas::{SiteStatisticsIn, Status};
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::DatabaseConnection;

#[rstest]
#[case::insert_correct("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec", "OK", 200)]
#[case::insert_incorrect_user("11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1", "User not found", 404)]
#[tokio::test]
async fn stat_site_endpoint(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] user_id: String,
    #[case] status: Status,
    #[case] status_code: u16,
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = test::init_service(App::new().app_data(Data::new(db)).service(stat_site)).await;
    let payload = SiteStatisticsIn {
        user_id: uuid::Uuid::parse_str(&user_id).unwrap(),
        endpoint: None,
    };
    let req = test::TestRequest::put()
        .uri("/site")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), status_code);
    let req = test::TestRequest::put()
        .uri("/site")
        .set_json(payload.clone())
        .to_request();
    let resp: Status = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp, status);
}
