use crate::app_state::AppState;
use crate::errors::{ApiError, ApiResult};
use crate::schemas::Status;
use actix_files::NamedFile;
use actix_web::{get, web};
use std::path::Path;

#[utoipa::path(
    get,
    path = "/api/review/image/{filename}",
    params(
        ("Api-Key" = inline(String), Header, minimum = 64, maximum = 64, example = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
        ("filename" = String, Path, description = "Path to file", example = "e3f295a9311d490888ad4706ad39220b.png"),
    ),
    responses(
        (
            status = 200, description = "Review image file", body = String,
            content_type = "image/png",
        ),
        (
            status = 404, description = "File not found", body = Status,
            example = json!(Status{status: "File not found".to_string()})
        )
    ),
    tag = "Review"
)]
#[get("/image/{filename:[a-f0-9]{32}\\.\\w{3,4}}")]
async fn get_image(
    state: web::Data<AppState>,
    filename: web::Path<String>,
) -> ApiResult<NamedFile> {
    let filename = filename.clone();
    let filename = Path::new(&filename)
        .file_name()
        .and_then(|v| v.to_str().to_owned());
    if filename.is_none() {
        Err(ApiError::UnprocessableData("Incorrect path".to_string()))?
    }
    let filename = filename.unwrap();
    let path = Path::new(&state.files_path).join(filename);
    if Path::exists(&path) {
        Ok(NamedFile::open_async(path).await?)
    } else {
        Err(ApiError::NotFound("File".to_owned()))
    }
}
