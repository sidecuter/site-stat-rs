use actix_web::{test, App};
use actix_web::web::Data;
use super::super::{prepare_connection, DATABASE_CONN};
use crate::api::get::user_id::get_user_id;
use rstest::*;

#[rstest]
async fn get_user_endpoint(
    #[future(awt)]
    prepare_connection: Result<(), Box<dyn std::error::Error>>
) {
    assert!(prepare_connection.is_ok());
    let db_lock = DATABASE_CONN.lock().unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(db_lock.as_ref().unwrap().clone()))
            .service(get_user_id)
    ).await;
    let req = test::TestRequest::get().uri("/user-id")
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{:?}", resp.response().body());
    assert_eq!(resp.status().as_u16(), 200)
}
