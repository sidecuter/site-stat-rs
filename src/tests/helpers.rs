use std::fs;
use std::path::Path;
use actix_web::http::header;
use actix_web::http::header::HeaderMap;
use actix_web::web::{BufMut, Bytes, BytesMut};
use mime::Mime;
use rand::distr::{Alphanumeric, SampleString};
use rand::rng;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;
use crate::app_state::AppState;
use crate::schemas::Problem;

pub fn prepare_tmp_dir() -> String {
    let filepath = format!("/tmp/{}", Uuid::new_v4());
    std::env::set_var("FILES_PATH", filepath.clone());
    let appstate = AppState::new();
    let files_path = Path::new(&appstate.files_path).join("images");
    if !files_path.exists() {
        fs::create_dir_all(files_path).unwrap();
    }
    filepath
}

pub async fn prepare_database(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    crate::entity::user_id::ActiveModel {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        creation_date: Set(chrono::Utc::now().naive_utc()),
    }
        .insert(db)
        .await?;
    crate::entity::site_stat::ActiveModel {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        endpoint: Set(None),
        ..Default::default()
    }
        .insert(db)
        .await?;
    crate::entity::select_aud::ActiveModel {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        auditory_id: Set("a-100".into()),
        success: Set(true),
        ..Default::default()
    }
        .insert(db)
        .await?;
    crate::entity::start_way::ActiveModel {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        start_id: Set("a-100".into()),
        end_id: Set("a-101".into()),
        ..Default::default()
    }
        .insert(db)
        .await?;
    crate::entity::change_plan::ActiveModel {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        visit_date: Set(chrono::Utc::now().naive_utc()),
        plan_id: Set("A-0".into()),
        ..Default::default()
    }
        .insert(db)
        .await?;
    crate::entity::review::ActiveModel {
        user_id: Set(Uuid::parse_str("11e1a4b8-7fa7-4501-9faa-541a5e0ff1ec")?),
        creation_date: Set(chrono::Utc::now().naive_utc()),
        text: Set("Awesome review".to_owned()),
        problem_id: Set("way".to_owned()),
        ..Default::default()
    }
        .insert(db)
        .await?;
    Ok(())
}

const CRLF: &[u8] = b"\r\n";
const CRLF_CRLF: &[u8] = b"\r\n\r\n";
const HYPHENS: &[u8] = b"--";
const BOUNDARY_PREFIX: &str = "------------------------";

pub const BLACK_1X1_PNG: &[u8] = &[
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
    file: Option<(String, Bytes, Option<Mime>)>
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
        if let Some(mimetype) = mimetype {
            buf.put(format!("Content-Type: {}", mimetype).as_bytes());
        } else {
            buf.put(CRLF);
        }
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
