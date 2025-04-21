use crate::api::stat::aud::stat_aud;
use crate::entity::select_aud;
use crate::schemas::SelectAuditoryIn;
use crate::tests::db::{add_aud, add_empty_row, add_exec_row, add_user_id, get_db};
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::{DbBackend, MockDatabase};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

#[rstest]
#[tokio::test]
async fn test_200_stat_aud_endpoint() {
    let db = Data::new(
        add_exec_row(add_aud(
            add_user_id(MockDatabase::new(DbBackend::Sqlite)),
            1,
        ))
        .append_query_results([[select_aud::Model {
            id: 0,
            user_id: uuid::Uuid::from_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
            visit_date: chrono::Utc::now().naive_utc(),
            auditory_id: "a-100".to_string(),
            success: true,
        }]])
        .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
        auditory_id: "a-100".to_string(),
        success: true,
    };
    let req = test::TestRequest::put()
        .uri("/select-aud")
        .set_json(payload.clone())
        .peer_addr(SocketAddr::new(IpAddr::from([192, 168, 1, 10]), 55050))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[rstest]
#[tokio::test]
async fn test_404_stat_aud_endpoint_user() {
    let db = Data::new(add_empty_row(MockDatabase::new(DbBackend::Sqlite)).into_connection());
    let app = test::init_service(App::new().app_data(db).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1").unwrap(),
        auditory_id: "a-100".to_string(),
        success: true,
    };
    let req = test::TestRequest::put()
        .uri("/select-aud")
        .set_json(payload.clone())
        .peer_addr(SocketAddr::new(IpAddr::from([192, 168, 1, 10]), 55050))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[rstest]
#[tokio::test]
async fn test_404_stat_aud_endpoint_aud() {
    let db = Data::new(
        add_empty_row(add_user_id(MockDatabase::new(DbBackend::Sqlite))).into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
        auditory_id: "a-100".to_string(),
        success: true,
    };
    let req = test::TestRequest::put()
        .uri("/select-aud")
        .set_json(payload.clone())
        .peer_addr(SocketAddr::new(IpAddr::from([192, 168, 1, 10]), 55050))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[rstest]
#[tokio::test]
async fn test_429_stat_aud_endpoint() {
    let app = test::init_service(App::new().app_data(get_db()).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1").unwrap(),
        auditory_id: "a-".into(),
        success: true,
    };
    let mut status: u16 = 200;
    for _ in 0..2 {
        let req = test::TestRequest::put()
            .uri("/select-aud")
            .set_json(payload.clone())
            .peer_addr(SocketAddr::new(IpAddr::from([192, 168, 1, 10]), 55050))
            .to_request();
        status = test::call_service(&app, req).await.status().into();
    }
    assert_eq!(status, 429);
}

#[rstest]
#[tokio::test]
async fn test_422_stat_aud_endpoint() {
    let app = test::init_service(App::new().app_data(get_db()).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: uuid::Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1").unwrap(),
        auditory_id: "a-".into(),
        success: true,
    };
    let req = test::TestRequest::put()
        .uri("/select-aud")
        .set_json(payload.clone())
        .peer_addr(SocketAddr::new(IpAddr::from([192, 168, 1, 1]), 55050))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 422);
}
