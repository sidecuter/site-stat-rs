use std::fs;
use super::super::prepare_connection;
use crate::api::review::add::add_review;
use crate::schemas::{Status, Problem};
use actix_web::web::{Bytes, Data};
use actix_web::{test, App};
use rstest::*;
use sea_orm::DatabaseConnection;
use crate::app_state::AppState;
use super::super::helpers::{prepare_tmp_dir, BLACK_1X1_PNG, generate_multipart_payload};

enum FileType {
    Image,
    Text,
    None,
    Invalid
}

#[rstest]
#[case::insert_correct(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    Problem::Way,
    "awesome text",
    FileType::None,
    "OK",
    200
)]
#[case::insert_incorrect_user(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1",
    Problem::Work,
    "awesome text",
    FileType::None,
    "User not found",
    404
)]
#[case::wrong_file_type(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    Problem::Plan,
    "awesome text",
    FileType::Text,
    "This endpoint accepts only images",
    415
)]
#[case::ok_with_file(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    Problem::Other,
    "awesome text",
    FileType::Image,
    "OK",
    200
)]
#[case::invalid_mime(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    Problem::Other,
    "awesome text",
    FileType::Invalid,
    "File has no mime type",
    422
)]
#[tokio::test]
async fn add_review_endpoint(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] user_id: String,
    #[case] problem: Problem,
    #[case] text: String,
    #[case] file_type: FileType,
    #[case] status: Status,
    #[case] status_code: u16,
) {
    assert!(prepare_connection.is_ok());
    let filepath = prepare_tmp_dir();
    let db = prepare_connection.unwrap();
    let app = test::init_service(App::new()
        .app_data(Data::new(db))
        .app_data(Data::new(AppState::default()))
        .service(add_review)
    ).await;
    let file = match file_type {
        FileType::Image => Some(
            ("image.png".to_string(), Bytes::from_static(BLACK_1X1_PNG), Some(mime::IMAGE_PNG))
        ),
        FileType::Text => Some(
            ("test.txt".to_string(), Bytes::from_static(b"Lorem ipsum."), Some(mime::TEXT_PLAIN))
        ),
        FileType::None => None,
        FileType::Invalid => Some(
            ("test.txt".to_string(), Bytes::from_static(b"Lorem ipsum."), None)
        )
    };
    let (payload, header) = generate_multipart_payload(user_id, problem, text, file);
    let req = test::TestRequest::post()
        .uri("/add");
    let req = header.clone()
        .into_iter()
        .fold(req, |req, hdr| req.insert_header(hdr))
        .set_payload(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), status_code);
    let req = test::TestRequest::post()
        .uri("/add");
    let req = header
        .into_iter()
        .fold(req, |req, hdr| req.insert_header(hdr))
        .set_payload(payload)
        .to_request();
    let resp: Status = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp, status);
    fs::remove_dir_all(filepath).unwrap()
}
