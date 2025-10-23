use crate::api::get::popular::get_popular;
use crate::tests::fixtures::prepare_connection;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::*;
use sea_orm::DatabaseConnection;

#[rstest]
#[actix_web::test]
async fn get_popular_endpoint(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(App::new().app_data(db).service(get_popular)).await;
    let req = test::TestRequest::get().uri("/popular").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200)
}
