use crate::api::get::stat::get_stat;
use crate::schemas::{FilterQuery, Target};
use crate::tests::db::FillDb;
use actix_web::web::Data;
use actix_web::{test, App};
use chrono::NaiveDate;
use rstest::*;
use sea_orm::{DbBackend, MockDatabase};

#[rstest]
#[case::site(Target::Site, None, None)]
#[case::auds(Target::Auds, None, None)]
#[case::ways(Target::Ways, None, None)]
#[case::plans(Target::Plans, None, None)]
#[case::site_with_start_date(Target::Site, Some(chrono::Utc::now().date_naive()), None)]
#[case::auds_with_start_date(Target::Auds, Some(chrono::Utc::now().date_naive()), None)]
#[case::ways_with_start_date(Target::Ways, Some(chrono::Utc::now().date_naive()), None)]
#[case::plans_with_start_date(Target::Plans, Some(chrono::Utc::now().date_naive()), None)]
#[case::site_with_both(
    Target::Site,
    Some(chrono::Utc::now().date_naive()),
    Some(chrono::Utc::now().date_naive())
)]
#[case::auds_with_both(
    Target::Auds,
    Some(chrono::Utc::now().date_naive()),
    Some(chrono::Utc::now().date_naive())
)]
#[case::ways_with_both(
    Target::Ways,
    Some(chrono::Utc::now().date_naive()),
    Some(chrono::Utc::now().date_naive())
)]
#[case::plans_with_both(
    Target::Plans,
    Some(chrono::Utc::now().date_naive()),
    Some(chrono::Utc::now().date_naive())
)]
#[actix_web::test]
async fn test_200_get_stat(
    #[case] target: Target,
    #[case] start_date: Option<NaiveDate>,
    #[case] end_date: Option<NaiveDate>,
) {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .add_count(3)
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(get_stat)).await;
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
}
