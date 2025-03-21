use super::super::prepare_connection;
use crate::api::review::add::add_review;
use crate::schemas::{Status, Problem};
use actix_web::web::{BufMut, Bytes, BytesMut, Data};
use actix_web::{test, App};
use actix_web::http::header;
use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
};
use actix_web::http::header::HeaderMap;
use rstest::*;
use sea_orm::DatabaseConnection;
use crate::app_state::AppState;

const CRLF: &[u8] = b"\r\n";
const CRLF_CRLF: &[u8] = b"\r\n\r\n";
const HYPHENS: &[u8] = b"--";
const BOUNDARY_PREFIX: &str = "------------------------";

pub fn generate_multipart_payload(
    user_id: String,
    problem: Problem,
    text: String,
    has_file: bool
) -> (Bytes, HeaderMap) {
    let filebuf = Bytes::from_static(b"Lorem ipsum.");
    let mut buf = if has_file {
        BytesMut::with_capacity(filebuf.len() + 128 + 330)
    } else {
        BytesMut::with_capacity(330)
    };
    let boundary = Alphanumeric.sample_string(&mut rng(), 32);
    let boundary_str = [BOUNDARY_PREFIX, &boundary].concat();
    let boundary = boundary_str.as_bytes();

    buf.put(HYPHENS);
    buf.put(boundary);
    buf.put(CRLF);

    buf.put("Content-Disposition: form-data; name=\"user_id\"".as_bytes());
    buf.put(CRLF_CRLF);

    buf.put(user_id.as_bytes());
    buf.put(CRLF);

    buf.put(HYPHENS);
    buf.put(boundary);
    buf.put(CRLF);
    buf.put("Content-Disposition: form-data; name=\"problem\"".as_bytes());
    buf.put(CRLF_CRLF);

    buf.put(problem.to_string().as_bytes());
    buf.put(CRLF);

    buf.put(HYPHENS);
    buf.put(boundary);
    buf.put(CRLF);
    buf.put("Content-Disposition: form-data; name=\"text\"".as_bytes());
    buf.put(CRLF_CRLF);

    buf.put(text.as_bytes());
    buf.put(CRLF);

    buf.put(HYPHENS);
    buf.put(boundary);
    if has_file {
        buf.put(CRLF);
        buf.put("Content-Disposition: form-data; name=\"image\"".as_bytes());
        buf.put("; filename=\"text.txt\"".as_bytes());
        buf.put(CRLF);
        buf.put(format!("Content-Type: {}", mime::TEXT_PLAIN).as_bytes());
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
    let _payload = String::from_utf8(bytes.to_vec()).unwrap();
    (bytes, headers)
}

#[rstest]
#[case::insert_correct(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    Problem::Way,
    "awesome text",
    false,
    "OK",
    200
)]
#[case::insert_incorrect_user(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1e1",
    Problem::Way,
    "awesome text",
    false,
    "User not found",
    404
)]
#[case::wrong_file_type(
    "11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec",
    Problem::Way,
    "awesome text",
    true,
    "This endpoint accepts only images",
    415
)]
#[tokio::test]
async fn add_review_endpoint(
    #[future(awt)] prepare_connection: Result<DatabaseConnection, Box<dyn std::error::Error>>,
    #[case] user_id: String,
    #[case] problem: Problem,
    #[case] text: String,
    #[case] has_file: bool,
    #[case] status: Status,
    #[case] status_code: u16,
) {
    assert!(prepare_connection.is_ok());
    let appstate = AppState::new();
    let db = prepare_connection.unwrap();
    let app = test::init_service(App::new().app_data(Data::new(db)).app_data(Data::new(appstate)).service(add_review)).await;
    let (payload, header) = generate_multipart_payload(user_id, problem, text, has_file);
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
}
