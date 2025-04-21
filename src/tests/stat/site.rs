use crate::api::stat::site::stat_site;
use crate::schemas::SiteStatisticsIn;
use crate::tests::db::{add_empty_row, add_exec_row, add_site, add_user_id, get_db};
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::{DbBackend, MockDatabase};
use utoipa::gen::serde_json::json;

#[rstest]
#[actix_web::test]
async fn test_200_stat_site() {
    let db = Data::new(
        add_exec_row(add_site(add_user_id(MockDatabase::new(DbBackend::Sqlite))))
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_site)).await;
    let payload = SiteStatisticsIn {
        user_id: Default::default(),
        endpoint: None,
    };
    let req = test::TestRequest::put()
        .uri("/site")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[rstest]
#[actix_web::test]
async fn test_404_stat_site_user() {
    let db = Data::new(add_empty_row(MockDatabase::new(DbBackend::Sqlite)).into_connection());
    let app = test::init_service(App::new().app_data(db).service(stat_site)).await;
    let payload = SiteStatisticsIn {
        user_id: Default::default(),
        endpoint: None,
    };
    let req = test::TestRequest::put()
        .uri("/site")
        .set_json(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[rstest]
#[actix_web::test]
async fn test_400_stat_site() {
    let app = test::init_service(App::new().app_data(get_db()).service(stat_site)).await;
    let req = test::TestRequest::put()
        .uri("/site")
        .set_json(json!({
            "user_id": "11e1a4b8-7fa7-4501-9faa-541a5e0ff1e"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
