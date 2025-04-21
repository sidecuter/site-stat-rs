use crate::api::get::user_id::get_user_id;
use crate::tests::db::{add_exec_row, add_user_id};
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::{DbBackend, MockDatabase};

#[rstest]
#[tokio::test]
async fn get_user_endpoint() {
    let db = Data::new(
        add_exec_row(add_user_id(MockDatabase::new(DbBackend::Sqlite))).into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(get_user_id)).await;
    let req = test::TestRequest::get().uri("/user-id").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200)
}
