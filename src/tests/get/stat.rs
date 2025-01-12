use actix_web::{test, App};
use actix_web::web::Data;
use super::super::prepare_connection;
use crate::api::get::stat::get_stat;
use rstest::*;
use sea_orm::DatabaseConnection;
use crate::schemas::Statistics;

#[rstest]
#[case::site(
    "Site",
    1
)]
#[case::auds(
    "Auds",
    1
)]
#[case::ways(
    "Ways",
    1
)]
#[case::plans(
    "Plans",
    1
)]
async fn get_stat_endpoint(
    #[future(awt)]
    prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] target: String,
    #[case] all: usize
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(db))
            .service(get_stat)
    ).await;
    let req = test::TestRequest::get()
        .uri(&format!(
            "/stat?api_key={}&target={}",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            target
        )).to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let req = test::TestRequest::get()
        .uri(&format!(
            "/stat?api_key={}&target={}",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            target
        )).to_request();
    let resp: Statistics = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.all, all);
}
