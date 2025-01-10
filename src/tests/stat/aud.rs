use actix_web::{test, App};
use actix_web::web::Data;
use rstest::*;
use sea_orm::DatabaseConnection;
use crate::api::stat::aud::stat_aud;
use crate::schemas::{SelectAuditoryIn, Status};
use super::super::prepare_connection;

#[rstest]
#[case::insert_correct(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    "a-100",
    true,
    "OK",
    200
)]
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
async fn stat_aud_endpoint(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] user_id: String,
    #[case] auditory_id: String,
    #[case] success: bool,
    #[case] status: Status,
    #[case] status_code: u16
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(db))
            .service(stat_aud)
    ).await;
    let payload = SelectAuditoryIn {
        user_id: uuid::Uuid::parse_str(&user_id).unwrap(),
        auditory_id: auditory_id.into(),
        success
    };
    let req = test::TestRequest::put().uri("/select-aud").set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), status_code);
    let req = test::TestRequest::put().uri("/select-aud").set_json(payload.clone())
        .to_request();
    let resp: Status = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp, status);
}
