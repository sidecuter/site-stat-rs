use crate::api::stat::aud::stat_aud;
use crate::entity::{aud, select_aud, user_id};
use crate::schemas::SelectAuditoryIn;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::{DatabaseConnection, DbBackend, MockDatabase, MockExecResult, MockRow};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

fn get_db() -> Data<DatabaseConnection> {
    Data::new(MockDatabase::new(DbBackend::Sqlite).into_connection())
}

#[rstest]
#[tokio::test]
async fn test_200_stat_aud_endpoint() {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .append_query_results([[user_id::Model {
                user_id: uuid::Uuid::from_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
                creation_date: chrono::Utc::now().naive_utc(),
            }]])
            .append_query_results([[aud::Model {
                id: "a-100".to_string(),
            }]])
            .append_query_results([[select_aud::Model {
                id: 0,
                user_id: uuid::Uuid::from_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
                visit_date: chrono::Utc::now().naive_utc(),
                auditory_id: "a-100".to_string(),
                success: true,
            }]])
            .append_exec_results([
                MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 1,
                },
            ])
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
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .append_query_results::<MockRow, Vec<_>, Vec<Vec<_>>>(vec![vec![]])
            .into_connection(),
    );
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
        MockDatabase::new(DbBackend::Sqlite)
            .append_query_results([[user_id::Model {
                user_id: uuid::Uuid::from_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec").unwrap(),
                creation_date: chrono::Utc::now().naive_utc(),
            }]])
            .append_query_results::<MockRow, Vec<_>, Vec<Vec<_>>>(vec![vec![]])
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
