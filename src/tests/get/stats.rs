use crate::api::get::{auds::get_auds, plans::get_plans, sites::get_sites, ways::get_ways};
use crate::api::review::get::get_reviews;
use crate::app_state::AppState;
use crate::schemas::{
    ChangePlanOut, Filter, Pagination, ReviewOut, SelectAuditoryOut, SiteStatisticsOut, StartWayOut,
};
use crate::tests::db::{FillDb, get_db};
use actix_web::web::Data;
use actix_web::{test, web, App};
use rstest::*;
use sea_orm::{DatabaseConnection, DbBackend, MockDatabase};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
enum Endpoint {
    Sites,
    Auds,
    Ways,
    Plans,
    Reviews,
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

pub fn get_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(get_sites)
            .service(get_auds)
            .service(get_ways)
            .service(get_plans)
            .service(get_reviews),
    );
}

fn get_db_filled(endpoint: Endpoint) -> Data<DatabaseConnection> {
    let mut mock = MockDatabase::new(DbBackend::Sqlite).add_count(2);
    mock = match endpoint {
        Endpoint::Sites => mock.add_site(),
        Endpoint::Auds => mock.add_select_add(),
        Endpoint::Ways => mock.add_start_way(),
        Endpoint::Plans => mock.add_change_plan(),
        Endpoint::Reviews => mock.add_review(),
    };
    Data::new(mock.into_connection())
}

#[rstest]
#[case::site_ok(Endpoint::Sites, false)]
#[case::auds_ok(Endpoint::Auds, false)]
#[case::ways_ok(Endpoint::Ways, false)]
#[case::plans_ok(Endpoint::Plans, false)]
#[case::reviews_ok(Endpoint::Reviews, false)]
#[case::site_filter(Endpoint::Sites, true)]
#[case::auds_filter(Endpoint::Auds, true)]
#[case::ways_filter(Endpoint::Ways, true)]
#[case::plans_filter(Endpoint::Plans, true)]
#[case::reviews_filter(Endpoint::Reviews, true)]
#[actix_web::test]
async fn test_200_get(#[case] endpoint: Endpoint, #[case] filter: bool) {
    let db = get_db_filled(endpoint);
    let app_state = Data::new(AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(app_state)
            .app_data(db)
            .configure(get_service),
    )
    .await;
    let query = Filter {
        user_id: if filter {
            Some(Default::default())
        } else {
            None
        },
        size: 50,
        page: 1,
    };
    let query = serde_qs::to_string(&query).unwrap();
    let req = test::TestRequest::get()
        .uri(&format!("/{endpoint}?{query}"))
        .insert_header((
            "Api-Key",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
}

#[rstest]
#[case::site_validation(Endpoint::Sites)]
#[case::auds_validation(Endpoint::Auds)]
#[case::ways_validation(Endpoint::Ways)]
#[case::plans_validation(Endpoint::Plans)]
#[case::reviews_validation(Endpoint::Reviews)]
#[actix_web::test]
async fn test_422_get(#[case] endpoint: Endpoint) {
    let db = get_db();
    let app_state = Data::new(AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(app_state)
            .app_data(db)
            .configure(get_service),
    )
    .await;
    let query = Filter {
        user_id: Default::default(),
        size: 50,
        page: 0,
    };
    let query = serde_qs::to_string(&query).unwrap();
    let req = test::TestRequest::get()
        .uri(&format!("/{endpoint}?{query}"))
        .insert_header((
            "Api-Key",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 422);
}

#[rstest]
#[case::site_notallowed(Endpoint::Sites)]
#[case::auds_notallowed(Endpoint::Auds)]
#[case::ways_notallowed(Endpoint::Ways)]
#[case::plans_notallowed(Endpoint::Plans)]
#[case::reviews_notallowed(Endpoint::Reviews)]
#[actix_web::test]
async fn test_403_get(#[case] endpoint: Endpoint) {
    let db = get_db();
    let app_state = Data::new(AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(app_state)
            .app_data(db)
            .configure(get_service),
    )
    .await;
    let query = Filter {
        user_id: Default::default(),
        size: 50,
        page: 1,
    };
    let query = serde_qs::to_string(&query).unwrap();
    let req = test::TestRequest::get()
        .uri(&format!("/{endpoint}?{query}"))
        .insert_header(("Api-Key", "1"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 403);
}

#[rstest]
#[case::site(Endpoint::Sites)]
#[case::auds(Endpoint::Auds)]
#[case::ways(Endpoint::Ways)]
#[case::plans(Endpoint::Plans)]
#[case::plans(Endpoint::Reviews)]
#[actix_web::test]
async fn check_value(#[case] endpoint: Endpoint) {
    let db = get_db_filled(endpoint);
    let app_state = Data::new(AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(app_state)
            .app_data(db)
            .configure(get_service),
    )
    .await;
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
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        ))
        .to_request();
    match endpoint {
        Endpoint::Sites => {
            let resp: Pagination<SiteStatisticsOut> =
                test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.total, 1);
        }
        Endpoint::Auds => {
            let resp: Pagination<SelectAuditoryOut> =
                test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.total, 1);
        }
        Endpoint::Ways => {
            let resp: Pagination<StartWayOut> = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.total, 1);
        }
        Endpoint::Plans => {
            let resp: Pagination<ChangePlanOut> = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.total, 1);
        }
        Endpoint::Reviews => {
            let resp: Pagination<ReviewOut> = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.total, 1);
        }
    }
}
