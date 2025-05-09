use crate::api::get::user_id::get_user_id;
use crate::tests::db::FillDb;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::{DbBackend, MockDatabase};

#[rstest]
#[actix_web::test]
async fn get_user_endpoint() {
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .add_user_id()
            .add_exec_row()
            .into_connection(),
    );
    let app = test::init_service(App::new().app_data(db).service(get_user_id)).await;
    let req = test::TestRequest::get().uri("/user-id").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200)
}
