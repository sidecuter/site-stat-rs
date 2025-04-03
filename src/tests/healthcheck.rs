use crate::api::healthcheck::get_status;
use actix_web::{test, App};
use rstest::*;

#[rstest]
#[tokio::test]
async fn get_user_endpoint() {
    let app = test::init_service(App::new().service(get_status)).await;
    let req = test::TestRequest::get().uri("/healthcheck").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200)
}
