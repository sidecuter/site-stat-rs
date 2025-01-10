use actix_web::{test, App};
use actix_web::web::Data;
use rstest::*;
use sea_orm::DatabaseConnection;
use crate::api::stat::way::stat_way;
use crate::schemas::{StartWayIn, Status};
use super::super::prepare_connection;

#[rstest]
#[case::insert_correct(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    "a-100",
    "a-101",
    "OK",
    200
)]
#[case::insert_incorrect_user(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1",
    "a-100",
    "a-101",
    "User not found",
    404
)]
#[case::insert_incorrect_start_id(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    "a-1000",
    "a-101",
    "Start auditory not found",
    404
)]
#[case::insert_incorrect_end_id(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    "a-100",
    "a-1011",
    "End auditory not found",
    404
)]
async fn stat_aud_endpoint(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] user_id: String,
    #[case] start_id: String,
    #[case] end_id: String,
    #[case] status: Status,
    #[case] status_code: u16
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(db))
            .service(stat_way)
    ).await;
    let payload = StartWayIn {
        user_id: uuid::Uuid::parse_str(&user_id).unwrap(),
        start_id: start_id.into(),
        end_id: end_id.into()
    };
    let req = test::TestRequest::put().uri("/start-way").set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), status_code);
    let req = test::TestRequest::put().uri("/start-way").set_json(payload.clone())
        .to_request();
    let resp: Status = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp, status);
}
