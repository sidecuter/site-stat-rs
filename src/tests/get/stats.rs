use crate::api::get::{
    auds::get_auds,
    plans::get_plans,
    sites::get_sites,
    ways::get_ways
};
use crate::schemas::{
    Pagination,
    SiteStatisticsOut,
    SelectAuditoryOut,
    StartWayOut,
    ChangePlanOut,
    ReviewOut
};
use rstest::*;
use actix_web::web::Data;
use actix_web::{test, App};
use sea_orm::DatabaseConnection;
use std::fmt::{Display, Formatter};
use super::super::prepare_connection;
use crate::api::review::get::get_reviews;

enum Endpoint {
    Sites,
    Auds,
    Ways,
    Plans,
    Reviews
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            Endpoint::Sites => "sites",
            Endpoint::Auds => "auds",
            Endpoint::Ways => "ways",
            Endpoint::Plans => "plans",
            Endpoint::Reviews => "get",
        };
        write!(f, "{}", a)
    }
}

#[rstest]
#[case::site_validation(
    Endpoint::Sites,
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    0,
    422
)]
#[case::auds_validation(
    Endpoint::Auds,
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    0,
    422
)]
#[case::ways_validation(
    Endpoint::Ways,
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    0,
    422
)]
#[case::plans_validation(
    Endpoint::Plans,
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    0,
    422
)]
#[case::reviews_validation(
    Endpoint::Reviews,
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    0,
    422
)]
#[case::site_ok(
    Endpoint::Sites,
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    1,
    200
)]
#[case::auds_ok(
    Endpoint::Auds,
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    1,
    200
)]
#[case::ways_ok(
    Endpoint::Ways,
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    1,
    200
)]
#[case::plans_ok(
    Endpoint::Plans,
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    1,
    200
)]
#[case::reviews_ok(
    Endpoint::Reviews,
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    1,
    200
)]
#[case::site_notallowed(Endpoint::Sites, "1", 0, 403)]
#[case::auds_notallowed(Endpoint::Auds, "1", 0, 403)]
#[case::ways_notallowed(Endpoint::Ways, "1", 0, 403)]
#[case::plans_notallowed(Endpoint::Plans, "1", 0, 403)]
#[case::reviews_notallowed(Endpoint::Reviews, "1", 0, 403)]
#[tokio::test]
async fn get(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] endpoint: Endpoint,
    #[case] key: &'static str,
    #[case] page: u64,
    #[case] status: u16
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = match endpoint {
        Endpoint::Sites => test::init_service(App::new().app_data(Data::new(db)).service(get_sites)),
        Endpoint::Auds => test::init_service(App::new().app_data(Data::new(db)).service(get_auds)),
        Endpoint::Ways => test::init_service(App::new().app_data(Data::new(db)).service(get_ways)),
        Endpoint::Plans => test::init_service(App::new().app_data(Data::new(db)).service(get_plans)),
        Endpoint::Reviews => test::init_service(App::new().app_data(Data::new(db)).service(get_reviews))
    }.await;
    let req = test::TestRequest::get()
        .uri(&format!("/{endpoint}?api_key={key}&page={page}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), status);
}


#[rstest]
#[case::site(Endpoint::Sites)]
#[case::auds(Endpoint::Auds)]
#[case::ways(Endpoint::Ways)]
#[case::plans(Endpoint::Plans)]
#[case::plans(Endpoint::Reviews)]
#[tokio::test]
async fn check_value(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] endpoint: Endpoint
) {
    assert!(prepare_connection.is_ok());
    let db = prepare_connection.unwrap();
    let app = match endpoint {
        Endpoint::Sites => test::init_service(App::new().app_data(Data::new(db)).service(get_sites)),
        Endpoint::Auds => test::init_service(App::new().app_data(Data::new(db)).service(get_auds)),
        Endpoint::Ways => test::init_service(App::new().app_data(Data::new(db)).service(get_ways)),
        Endpoint::Plans => test::init_service(App::new().app_data(Data::new(db)).service(get_plans)),
        Endpoint::Reviews => test::init_service(App::new().app_data(Data::new(db)).service(get_reviews))
    }.await;
    let req = test::TestRequest::get()
        .uri(&format!(
            "/{endpoint}?api_key={}",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        ))
        .to_request();
    match endpoint {
        Endpoint::Sites => {
            let resp: Pagination<SiteStatisticsOut> = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.total, 1);
        },
        Endpoint::Auds => {
            let resp: Pagination<SelectAuditoryOut> = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.total, 1);
        },
        Endpoint::Ways => {
            let resp: Pagination<StartWayOut> = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.total, 1);
        },
        Endpoint::Plans => {
            let resp: Pagination<ChangePlanOut> = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.total, 1);
        },
        Endpoint::Reviews => {
            let resp: Pagination<ReviewOut> = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.total, 1);
        }
    }
}
