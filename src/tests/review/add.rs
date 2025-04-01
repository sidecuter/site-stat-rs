use std::fs;
use super::super::prepare_connection;
use crate::api::review::add::add_review;
use crate::schemas::{Status, Problem};
use actix_web::web::{BufMut, Bytes, BytesMut, Data};
use actix_web::{test, App};
use actix_web::http::header;
use rand::{distr::{Alphanumeric, SampleString}, rng};
use actix_web::http::header::HeaderMap;
use mime::Mime;
use rstest::*;
use sea_orm::DatabaseConnection;
use crate::app_state::AppState;

const CRLF: &[u8] = b"\r\n";
const CRLF_CRLF: &[u8] = b"\r\n\r\n";
const HYPHENS: &[u8] = b"--";
const BOUNDARY_PREFIX: &str = "------------------------";

const BLACK_1X1_PNG: &[u8] = &[
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
    0x00, 0x00, 0x00, 0x0D,
    0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x01,
    0x08,
    0x00,
    0x00,
    0x00,
    0x00,
    0x8C, 0x69, 0x82, 0x8A,
    0x00, 0x00, 0x00, 0x0C,
    0x49, 0x44, 0x41, 0x54,
    0x78, 0x01,
    0x01, 0x02, 0x00, 0xFD,
    0xFF, 0x00, 0x00, 0x01,
    0x00, 0x01,
    0x9A, 0x1C, 0x21, 0xBC,
    0x00, 0x00, 0x00, 0x00,
    0x49, 0x45, 0x4E, 0x44,
    0xAE, 0x42, 0x60, 0x82,
];

pub fn generate_multipart_payload(
    user_id: String,
    problem: Problem,
    text: String,
    file: Option<(String, Bytes, Mime)>
) -> (Bytes, HeaderMap) {
    let boundary = Alphanumeric.sample_string(&mut rng(), 32);
    let boundary_str = [BOUNDARY_PREFIX, &boundary].concat();
    let boundary = boundary_str.as_bytes();
    let sub = |buf: &mut BytesMut, name, val: String| {
        buf.put(CRLF);
        buf.put(format!("Content-Disposition: form-data; name=\"{name}\"").as_bytes());
        buf.put(CRLF_CRLF);
        buf.put(val.as_bytes());
        buf.put(CRLF);
        buf.put(HYPHENS);
        buf.put(boundary);
    };
    let mut buf = if let Some((_, filebuf, _)) = file.clone() {
        BytesMut::with_capacity(filebuf.len() + 128 + 330)
    } else {
        BytesMut::with_capacity(330)
    };

    buf.put(HYPHENS);
    buf.put(boundary);
    sub(&mut buf, "user_id", user_id);
    sub(&mut buf, "problem", problem.to_string());
    sub(&mut buf, "text", text);
    if let Some((filename, filebuf, mimetype)) = file {
        buf.put(CRLF);
        buf.put("Content-Disposition: form-data; name=\"image\"".as_bytes());
        buf.put(format!("; filename=\"{filename}\"",).as_bytes());
        buf.put(CRLF);
        buf.put(format!("Content-Type: {}", mimetype).as_bytes());
        buf.put(CRLF_CRLF);

        buf.put(filebuf);
        buf.put(CRLF_CRLF);

        buf.put(HYPHENS);
        buf.put(boundary);
    }
    buf.put(HYPHENS);
    buf.put(CRLF);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        format!("multipart/form-data; boundary=\"{boundary_str}\"").parse().unwrap()
    );
    let bytes = buf.freeze();
    (bytes, headers)
}

enum FileType {
    Image,
    Text,
    None
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
    Problem::Way,
    "awesome text",
    FileType::None,
    "User not found",
    404
)]
#[case::wrong_file_type(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    Problem::Way,
    "awesome text",
    FileType::Text,
    "This endpoint accepts only images",
    415
)]
#[case::ok_with_file(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    Problem::Way,
    "awesome text",
    FileType::Image,
    "OK",
    200
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
    let filepath = format!("/tmp/{}", uuid::Uuid::new_v4());
    std::env::set_var("FILES_PATH", filepath.clone());
    let appstate = AppState::new();
    if !std::path::Path::new(&appstate.files_path).exists() {
        fs::create_dir(appstate.files_path.clone()).unwrap();
    }
    let db = prepare_connection.unwrap();
    let app = test::init_service(App::new().app_data(Data::new(db)).app_data(Data::new(appstate)).service(add_review)).await;
    let file = match file_type {
        FileType::Image => Some(
            ("image.png".to_string(), Bytes::from_static(BLACK_1X1_PNG), mime::IMAGE_PNG)
        ),
        FileType::Text => Some(
            ("test.txt".to_string(), Bytes::from_static(b"Lorem ipsum."), mime::TEXT_PLAIN)
        ),
        FileType::None => None,
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
