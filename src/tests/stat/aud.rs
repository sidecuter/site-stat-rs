use crate::api::stat::aud::stat_aud;
use crate::schemas::SelectAuditoryIn;
use crate::tests::db::{get_db, FillDb};
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::{DbBackend, MockDatabase};
use std::net::{IpAddr, SocketAddr};

#[rstest]
#[actix_web::test]
async fn test_200_stat_aud_endpoint() {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .add_user_id()
            .add_aud(1)
            .add_select_add()
            .add_exec_row()
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: Default::default(),
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
#[actix_web::test]
async fn test_404_stat_aud_endpoint_user() {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .add_empty_row()
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: Default::default(),
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
#[actix_web::test]
async fn test_404_stat_aud_endpoint_aud() {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .add_user_id()
            .add_empty_row()
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: Default::default(),
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
#[actix_web::test]
async fn test_429_stat_aud_endpoint() {
    let app = test::init_service(App::new().app_data(get_db()).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: Default::default(),
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
#[actix_web::test]
async fn test_422_stat_aud_endpoint() {
    let app = test::init_service(App::new().app_data(get_db()).service(stat_aud)).await;
    let payload = SelectAuditoryIn {
        user_id: Default::default(),
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
