use std::fs;
use std::io::Write;
use std::path::Path;
use actix_web::{test, App};
use actix_web::web::Data;
use rstest::rstest;
use uuid::Uuid;
use crate::api::review::image::get_image;
use crate::app_state::AppState;
use super::add::BLACK_1X1_PNG;

#[rstest]
#[tokio::test]
async fn get_image_endpoint() {
    let filepath = format!("/tmp/{}", Uuid::new_v4());
    let filename = format!("{}.png", Uuid::new_v4());
    std::env::set_var("FILES_PATH", filepath.clone());
    let appstate = AppState::new();
    if !Path::new(&appstate.files_path).exists() {
        fs::create_dir(appstate.files_path.clone()).unwrap();
    }
    {
        let mut file = fs::File::create(Path::new(&filepath).join(filename.clone())).unwrap();
        file.write_all(BLACK_1X1_PNG).unwrap();
    }
    let app = test::init_service(App::new().app_data(Data::new(appstate)).service(get_image)).await;
    let req = test::TestRequest::get().uri(&format!("/image/{}", filename)).to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    fs::remove_dir_all(filepath).unwrap()
}
