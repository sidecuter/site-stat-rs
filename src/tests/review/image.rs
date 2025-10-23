use super::super::helpers::prepare_tmp_dir;
use super::super::helpers::BLACK_1X1_PNG;
use crate::api::review::image::get_image;
use crate::config::AppConfig;
use crate::tests::db::FillDb;
use crate::tests::fixtures::jwt_token;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::rstest;
use sea_orm::{DatabaseBackend, MockDatabase};
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

#[rstest]
#[actix_web::test]
async fn get_image_endpoint(jwt_token: &String) {
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
            .app_data(Data::new(
                MockDatabase::new(DatabaseBackend::Sqlite)
                    .add_user_roles()
                    .into_connection(),
            ))
            .service(get_image),
    )
    .await;
    let req = test::TestRequest::get()
        .uri(&format!("/image/{}", filename))
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    fs::remove_dir_all(filepath).unwrap()
}
