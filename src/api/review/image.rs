use actix_files::NamedFile;
use actix_web::{get, web};
use crate::schemas::Status;
use crate::app_state::AppState;
use crate::errors::{Result as ApiResult, Error as ApiError};

#[utoipa::path(
    get,
    path = "/api/review/image/{filename:.*}",
    params(
        ("filename" = String, Path, description = "Path to file"),
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
#[get("/image/{filename:.*}")]
async fn get_file(
    state: web::Data<AppState>,
    filename: web::Path<String>
) -> ApiResult<NamedFile> {
    let path = std::path::Path::new(&state.files_path)
        .join(&filename.clone());
    if std::path::Path::exists(&path) {
        Ok(NamedFile::open_async(path).await?)
    } else {
        Err(ApiError::NotFound("File".to_owned()))
    }
}
