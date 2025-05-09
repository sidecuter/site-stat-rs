use super::super::helpers::prepare_tmp_dir;
use super::super::helpers::BLACK_1X1_PNG;
use crate::api::review::image::get_image;
use crate::config::AppConfig;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::rstest;
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

#[rstest]
#[actix_web::test]
async fn get_image_endpoint() {
    let filename = format!("{}.png", Uuid::new_v4().to_string().replace("-", ""));
    let filepath = prepare_tmp_dir();
    {
        let mut file =
            fs::File::create(Path::new(&filepath).join("images").join(filename.clone())).unwrap();
        file.write_all(BLACK_1X1_PNG).unwrap();
    }
    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppConfig::default()))
            .service(get_image),
    )
    .await;
    let req = test::TestRequest::get()
        .uri(&format!("/image/{}", filename))
        .insert_header((
            "Api-Key",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcde1",
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    fs::remove_dir_all(filepath).unwrap()
}
