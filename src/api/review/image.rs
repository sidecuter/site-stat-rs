use crate::config::AppConfig;
use crate::errors::{ApiError, ApiResult};
use crate::schemas::{rights, goals, Status};
use actix_files::NamedFile;
use actix_web::{get, web};
use crate::auth::IsCapable;

#[utoipa::path(
    get,
    path = "/api/review/image/{filename}",
    params(
        ("Authorization" = inline(String), Header, minimum = 7, example = "Bearer <token>"),
        ("filename" = String, Path, description = "Path to file", example = "e3f295a9311d490888ad4706ad39220b.png"),
    ),
    responses(
        (
            status = 200, description = "Review image file", body = String,
            content_type = "image/png",
        ),
        (
            status = 401, description = "User is inactive or not present", body = Status,
            example = json!(Status{status: "User is inactive or not present".to_string()})
        ),
        (
            status = 404, description = "File not found", body = Status,
            example = json!(Status{status: "File not found".to_string()})
        )
    ),
    security(
        ("oauth2_bearer" = ["view::reviews"])
    ),
    tag = "Review"
)]
#[get("/image/{filename:[a-f0-9]{32}\\.\\w{3,4}}")]
async fn get_image(
    config: web::Data<AppConfig>,
    filename: web::Path<String>,
    _is_capable: IsCapable<rights::View, goals::Reviews>
) -> ApiResult<NamedFile> {
    let filename = filename.clone();
    let filename = std::path::Path::new(&filename)
        .file_name()
        .and_then(|v| v.to_str());
    if filename.is_none() {
        Err(ApiError::UnprocessableData("Incorrect path".to_string()))?;
    }
    let filename = filename.unwrap();
    let path = config.get_files_path().join(filename);
    if path.exists() {
        Ok(NamedFile::open_async(path).await?)
    } else {
        Err(ApiError::NotFound("File".to_owned()))
    }
}
