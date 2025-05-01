#![allow(clippy::clone_on_copy)]

use crate::config::AppConfig;
use crate::entity::user_id;
use crate::errors::ApiResult;
use crate::schemas::review::ReviewFormIn;
use crate::schemas::{ReviewIn, Status};
use crate::traits::{ConversionToStatusTrait, FilterTrait};
use actix_multipart::form::MultipartForm;
use actix_web::{post, web};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};

#[utoipa::path(
    post,
    path = "/api/review/add",
    request_body(content = ReviewFormIn, content_type = "multipart/form-data"),
    responses(
        (
            status = 200, description = "Review added", body = Status,
            example = json!(Status::default())
        ),
        (
            status = 404, description = "User not found", body = Status,
            example = json!(Status{status: "Auditory not found".to_string()})
        ),
        (
            status = 415, description = "File type not supported", body = Status,
            example = json!(Status{status: "This endpoint accepts only images".to_string()})
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        ),
    ),
    tag = "Review"
)]
#[post("add")]
async fn add_review(
    config: web::Data<AppConfig>,
    MultipartForm(data): MultipartForm<ReviewFormIn>,
    db: web::Data<DatabaseConnection>,
) -> ApiResult<Status> {
    user_id::Entity::filter(data.user_id.clone(), db.get_ref(), "User".to_string()).await?;
    let mut review_in = ReviewIn {
        user_id: data.user_id.clone(),
        text: data.text.clone(),
        problem: data.problem.clone(),
        ..Default::default()
    };
    let image_name = data.save_image(&config).await?;
    review_in.image_name = image_name;
    review_in
        .into_active_model()
        .insert(db.get_ref())
        .await
        .status_ok()
}
