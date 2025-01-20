use std::net::{IpAddr, SocketAddr};
use super::super::prepare_connection;
use crate::api::stat::aud::stat_aud;
use crate::schemas::{SelectAuditoryIn, Status};
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::DatabaseConnection;

#[rstest]
#[case::insert_correct("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec", "a-100", true, "OK", 200)]
#[case::insert_incorrect_user(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1",
    "a-100",
    true,
    "User not found",
    404
)]
#[case::insert_incorrect_auditory(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    "a-1000",
    true,
    "Auditory not found",
    404
)]
#[tokio::test]
async fn stat_aud_endpoint(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] user_id: String,
    #[case] auditory_id: String,
    #[case] success: bool,
    #[case] status: Status,
    #[case] status_code: u16,
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = test::init_service(App::new().app_data(Data::new(db)).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: uuid::Uuid::parse_str(&user_id).unwrap(),
        auditory_id: auditory_id.into(),
        success,
    };
    let req = test::TestRequest::put()
        .uri("/select-aud")
        .set_json(payload.clone())
        .peer_addr(SocketAddr::new(IpAddr::from([192, 168, 1, 1]), 55050))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), status_code);
    let req = test::TestRequest::put()
        .uri("/select-aud")
        .set_json(payload.clone())
        .peer_addr(SocketAddr::new(IpAddr::from([192, 168, 1, 2]), 55050))
        .to_request();
    let resp: Status = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp, status);
}

#[rstest]
#[tokio::test]
async fn test_429_stat_aud_endpoint (
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = test::init_service(App::new().app_data(Data::new(db)).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1").unwrap(),
        auditory_id: "a-100".into(),
        success: true,
    };
    let mut status: u16 = 200;
    for _ in 0..2 {
        let req = test::TestRequest::put()
            .uri("/select-aud")
            .set_json(payload.clone())
            .peer_addr(SocketAddr::new(IpAddr::from([192, 168, 1, 10]), 55050))
            .to_request();
        status = test::call_service(&app, req).await.status().into();
    }
    assert_eq!(status, 429);
}
