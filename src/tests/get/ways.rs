use super::super::prepare_connection;
use crate::api::get::ways::get_ways;
use crate::schemas::{Pagination, StartWayOut};
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::DatabaseConnection;

#[rstest]
#[tokio::test]
async fn get_ways_endpoint(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = test::init_service(App::new().app_data(Data::new(db)).service(get_ways)).await;
    let req = test::TestRequest::get()
        .uri(&format!(
            "/ways?api_key={}",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let req = test::TestRequest::get()
        .uri(&format!(
            "/ways?api_key={}",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        ))
        .to_request();
    let resp: Pagination<StartWayOut> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.total, 1);
}
