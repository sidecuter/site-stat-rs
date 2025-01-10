use actix_web::{test, App};
use actix_web::web::Data;
use rstest::*;
use sea_orm::DatabaseConnection;
use crate::api::stat::plan::stat_plan;
use crate::schemas::{ChangePlanIn, Status};
use super::super::prepare_connection;

#[rstest]
#[case::insert_correct(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    "A-0",
    "OK",
    200
)]
#[case::insert_incorrect_user(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1",
    "A-0",
    "User not found",
    404
)]
#[case::insert_incorrect_plan(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    "A-8",
    "Changed plan not found",
    404
)]
async fn stat_aud_endpoint(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] user_id: String,
    #[case] plan_id: String,
    #[case] status: Status,
    #[case] status_code: u16
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(db))
            .service(stat_plan)
    ).await;
    let payload = ChangePlanIn {
        user_id: uuid::Uuid::parse_str(&user_id).unwrap(),
        plan_id: plan_id.into()
    };
    let req = test::TestRequest::put().uri("/change-plan").set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), status_code);
    let req = test::TestRequest::put().uri("/change-plan").set_json(payload.clone())
        .to_request();
    let resp: Status = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp, status);
}
