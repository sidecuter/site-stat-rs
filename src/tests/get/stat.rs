use super::super::prepare_connection;
use crate::api::get::stat::get_stat;
use crate::schemas::Statistics;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::DatabaseConnection;

#[rstest]
#[case::site("site", 1)]
#[case::auds("auds", 1)]
#[case::ways("ways", 1)]
#[case::plans("plans", 1)]
#[tokio::test]
async fn get_stat_endpoint(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] target: String,
    #[case] all: u64,
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = test::init_service(App::new().app_data(Data::new(db)).service(get_stat)).await;
    let req = test::TestRequest::get()
        .uri(&format!(
            "/stat?api_key={}&target={}",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef", target
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let req = test::TestRequest::get()
        .uri(&format!(
            "/stat?api_key={}&target={}",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef", target
        ))
        .to_request();
    let resp: Statistics = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.all, all);
}
