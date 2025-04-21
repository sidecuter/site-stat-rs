use crate::api::get::user_id::get_user_id;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::{DbBackend, MockDatabase, MockExecResult};
use crate::entity::user_id;

#[rstest]
#[tokio::test]
async fn get_user_endpoint() {
    let db = Data::new(MockDatabase::new(DbBackend::Sqlite)
        .append_query_results([
            vec![
                user_id::Model {
                    user_id: uuid::Uuid::new_v4(),
                    creation_date: chrono::Utc::now().naive_utc()
                }
            ]
        ])
        .append_exec_results([
            MockExecResult {
                last_insert_id: 0,
                rows_affected: 1
            }
        ])
        .into_connection());
    let app = test::init_service(App::new().app_data(db).service(get_user_id)).await;
    let req = test::TestRequest::get().uri("/user-id").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200)
}
