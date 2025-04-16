use crate::api::get::{
    auds::get_auds,
    plans::get_plans,
    sites::get_sites,
    ways::get_ways
};
use crate::schemas::{
    Pagination, SiteStatisticsOut, SelectAuditoryOut,
    StartWayOut, ChangePlanOut, ReviewOut, Filter
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
#[case::site_validation(Endpoint::Sites, true, 0, 422, false)]
#[case::auds_validation(Endpoint::Auds, true, 0, 422, false)]
#[case::ways_validation(Endpoint::Ways, true, 0, 422, false)]
#[case::plans_validation(Endpoint::Plans, true, 0, 422, false)]
#[case::reviews_validation(Endpoint::Reviews, true, 0, 422, false)]
#[case::site_ok(Endpoint::Sites, true, 1, 200, false)]
#[case::auds_ok(Endpoint::Auds, true, 1, 200, false)]
#[case::ways_ok(Endpoint::Ways, true, 1, 200, false)]
#[case::plans_ok(Endpoint::Plans, true, 1, 200, false)]
#[case::reviews_ok(Endpoint::Reviews, true, 1, 200, false)]
#[case::site_filter(Endpoint::Sites, true, 1, 200, true)]
#[case::auds_filter(Endpoint::Auds, true, 1, 200, true)]
#[case::ways_filter(Endpoint::Ways, true, 1, 200, true)]
#[case::plans_filter(Endpoint::Plans, true, 1, 200, true)]
#[case::reviews_filter(Endpoint::Reviews, true, 1, 200, true)]
#[case::site_notallowed(Endpoint::Sites, false, 0, 403, false)]
#[case::auds_notallowed(Endpoint::Auds, false, 0, 403, false)]
#[case::ways_notallowed(Endpoint::Ways, false, 0, 403, false)]
#[case::plans_notallowed(Endpoint::Plans, false, 0, 403, false)]
#[case::reviews_notallowed(Endpoint::Reviews, false, 0, 403, false)]
#[tokio::test]
async fn get(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] endpoint: Endpoint,
    #[case] correct: bool,
    #[case] page: u64,
    #[case] status: u16,
    #[case] filter: bool
) {
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app_state = Data::new(crate::app_state::AppState::new());
    let app = match endpoint {
        Endpoint::Sites => test::init_service(App::new().app_data(app_state).app_data(db).service(get_sites)),
        Endpoint::Auds => test::init_service(App::new().app_data(app_state).app_data(db).service(get_auds)),
        Endpoint::Ways => test::init_service(App::new().app_data(app_state).app_data(db).service(get_ways)),
        Endpoint::Plans => test::init_service(App::new().app_data(app_state).app_data(db).service(get_plans)),
        Endpoint::Reviews => test::init_service(App::new().app_data(app_state).app_data(db).service(get_reviews))
    }.await;
    let query = Filter {
        user_id: if filter {
            Some(uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap())
        } else { None },
        size: 50,
        page,
    };
    let query = serde_qs::to_string(&query).unwrap();
    let req = test::TestRequest::get()
        .uri(&format!("/{endpoint}?{query}"))
        .insert_header(
            (
                "Api-Key",
                if correct {"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"} else {"1"}
            ))
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
    let query = Filter {
        user_id: None,
        size: 50,
        page: 1,
    };
    let query = serde_qs::to_string(&query).unwrap();
    let req = test::TestRequest::get()
        .uri(&format!("/{endpoint}?{query}"))
        .insert_header((
            "Api-Key",
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
