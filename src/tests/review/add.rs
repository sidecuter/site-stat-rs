use super::super::helpers::{generate_multipart_payload, prepare_tmp_dir, BLACK_1X1_PNG};
use crate::api::review::add;
use crate::config::AppConfig;
use crate::schemas::Problem;
use crate::tests::fixtures::prepare_connection;
use actix_web::web::{Bytes, Data};
use actix_web::{test, App};
use rstest::*;
use sea_orm::DatabaseConnection;
use std::fs;

#[rstest]
#[actix_web::test]
async fn test_200_add_review(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(
        App::new()
            .app_data(db)
            .app_data(Data::new(AppConfig::default()))
            .service(add::add_review),
    )
    .await;
    let (payload, header) = generate_multipart_payload(
        "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec".to_string(),
        Problem::Way,
        "awesome text".to_string(),
        None,
    );
    let req = test::TestRequest::post().uri("/add");
    let req = header
        .clone()
        .into_iter()
        .fold(req, |req, hdr| req.insert_header(hdr))
        .set_payload(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[rstest]
#[actix_web::test]
async fn test_200_add_review_with_image(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let filepath = prepare_tmp_dir();
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(
        App::new()
            .app_data(db)
            .app_data(Data::new(AppConfig::default()))
            .service(add::add_review),
    )
    .await;
    let (payload, header) = generate_multipart_payload(
        "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec".to_string(),
        Problem::Other,
        "awesome text".to_string(),
        Some((
            "image.png".to_string(),
            Bytes::from_static(BLACK_1X1_PNG),
            Some(mime::IMAGE_PNG),
        )),
    );
    let req = test::TestRequest::post().uri("/add");
    let req = header
        .clone()
        .into_iter()
        .fold(req, |req, hdr| req.insert_header(hdr))
        .set_payload(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    fs::remove_dir_all(filepath).unwrap()
}

#[rstest]
#[actix_web::test]
async fn test_404_add_review_user(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(
        App::new()
            .app_data(db)
            .app_data(Data::new(AppConfig::default()))
            .service(add::add_review),
    )
    .await;
    let (payload, header) = generate_multipart_payload(
        "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ed".to_string(),
        Problem::Work,
        "awesome text".to_string(),
        None,
    );
    let req = test::TestRequest::post().uri("/add");
    let req = header
        .clone()
        .into_iter()
        .fold(req, |req, hdr| req.insert_header(hdr))
        .set_payload(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[rstest]
#[actix_web::test]
async fn test_415_add_review(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(
        App::new()
            .app_data(db)
            .app_data(Data::new(AppConfig::default()))
            .service(add::add_review),
    )
    .await;
    let (payload, header) = generate_multipart_payload(
        "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec".to_string(),
        Problem::Plan,
        "awesome text".to_string(),
        Some((
            "test.txt".to_string(),
            Bytes::from_static(b"Lorem ipsum."),
            Some(mime::TEXT_PLAIN),
        )),
    );
    let req = test::TestRequest::post().uri("/add");
    let req = header
        .clone()
        .into_iter()
        .fold(req, |req, hdr| req.insert_header(hdr))
        .set_payload(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 415);
}

#[rstest]
#[actix_web::test]
async fn test_422_add_review(
    #[future] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
) {
    let prepare_connection = prepare_connection.await;
    assert!(prepare_connection.is_ok());
    let db = Data::new(prepare_connection.unwrap());
    let app = test::init_service(
        App::new()
            .app_data(db)
            .app_data(Data::new(AppConfig::default()))
            .service(add::add_review),
    )
    .await;
    let (payload, header) = generate_multipart_payload(
        "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec".to_string(),
        Problem::Other,
        "awesome text".to_string(),
        Some((
            "test.txt".to_string(),
            Bytes::from_static(b"Lorem ipsum."),
            None,
        )),
    );
    let req = test::TestRequest::post().uri("/add");
    let req = header
        .clone()
        .into_iter()
        .fold(req, |req, hdr| req.insert_header(hdr))
        .set_payload(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 422);
}
