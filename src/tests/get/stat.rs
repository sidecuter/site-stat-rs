use super::super::prepare_connection;
use crate::api::get::stat::get_stat;
use crate::schemas::{FilterQuery, Statistics, Target};
use actix_web::web::Data;
use actix_web::{test, App};
use chrono::NaiveDate;
use rstest::*;
use sea_orm::DatabaseConnection;

#[rstest]
#[case::site(Target::Site, 1, None, None)]
#[case::auds(Target::Auds, 1, None, None)]
#[case::ways(Target::Ways, 1, None, None)]
#[case::plans(Target::Plans, 1, None, None)]
#[case::site_with_start_date(Target::Site, 1, Some(chrono::Utc::now().date_naive()), None)]
#[case::auds_with_start_date(Target::Auds, 1, Some(chrono::Utc::now().date_naive()), None)]
#[case::ways_with_start_date(Target::Ways, 1, Some(chrono::Utc::now().date_naive()), None)]
#[case::plans_with_start_date(Target::Plans, 1, Some(chrono::Utc::now().date_naive()), None)]
#[case::site_with_both(
    Target::Site, 1,
    Some(chrono::Utc::now().date_naive()),
    Some(chrono::Utc::now().date_naive())
)]
#[case::auds_with_both(
    Target::Auds, 1,
    Some(chrono::Utc::now().date_naive()),
    Some(chrono::Utc::now().date_naive())
)]
#[case::ways_with_both(
    Target::Ways, 1,
    Some(chrono::Utc::now().date_naive()),
    Some(chrono::Utc::now().date_naive())
)]
#[case::plans_with_both(
    Target::Plans, 1,
    Some(chrono::Utc::now().date_naive()),
    Some(chrono::Utc::now().date_naive())
)]
#[tokio::test]
async fn get_stat_endpoint(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] target: Target,
    #[case] all: u64,
    #[case] start_date: Option<NaiveDate>,
    #[case] end_date: Option<NaiveDate>,
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = test::init_service(App::new().app_data(Data::new(db)).service(get_stat)).await;
    let query = FilterQuery {
        target,
        start_date,
        end_date,
    };
    let query = serde_qs::to_string(&query).unwrap();
    let req = test::TestRequest::get()
        .uri(&format!("/stat?{query}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let req = test::TestRequest::get()
        .uri(&format!("/stat?{query}"))
        .to_request();
    let resp: Statistics = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.all, all);
}
