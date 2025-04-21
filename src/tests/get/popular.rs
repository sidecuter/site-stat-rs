use std::collections::BTreeMap;
use crate::api::get::popular::get_popular;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::{DbBackend, MockDatabase, Value};

#[rstest]
#[actix_web::test]
async fn get_popular_endpoint() {
    let mut map = BTreeMap::new();
    map.insert("ID".to_string(), Value::String(Some(Box::new("a-100".to_string()))));
    let db = Data::new(
        MockDatabase::new(DbBackend::Sqlite)
            .append_query_results([vec![map]])
            .into_connection()
    );
    let app = test::init_service(App::new().app_data(db).service(get_popular)).await;
    let req = test::TestRequest::get().uri("/popular").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200)
}
