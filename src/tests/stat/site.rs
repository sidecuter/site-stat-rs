use actix_web::{test, App};
use actix_web::web::Data;
use rstest::*;
use crate::api::stat::site::stat_site;
use crate::schemas::{SiteStatisticsIn, Status};
use super::super::{prepare_connection, DATABASE_CONN};

#[rstest]
#[case::insert_correct(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    "OK",
    200
)]
#[case::insert_incorrect_user(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1",
    "User not found",
    404
)]
async fn stat_site_endpoint(
    #[future(awt)]
    prepare_connection: Result<(), Box<dyn std::error::Error>>,
    #[case] user_id: String,
    #[case] status: Status,
    #[case] status_code: u16
) {
    assert!(prepare_connection.is_ok());
    let db_lock = DATABASE_CONN.lock().unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(db_lock.as_ref().unwrap().clone()))
            .service(stat_site)
    ).await;
    let payload = SiteStatisticsIn {
        user_id: uuid::Uuid::parse_str(&user_id).unwrap(),
        endpoint: None,
    };
    let req = test::TestRequest::put().uri("/site").set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), status_code);
    let req = test::TestRequest::put().uri("/site").set_json(payload.clone())
        .to_request();
    let resp: Status = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp, status);
}
