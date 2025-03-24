use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};
use actix_web::{put, web};
use validator::Validate;
use crate::traits::{ConversionToStatusTrait, FilterTrait};
use crate::schemas::{StartWayIn, Status};
use crate::entity::{aud, user_id};
use crate::errors::{ApiError, ApiResult};

#[utoipa::path(
    put,
    path = "/api/stat/start-way",
    request_body = StartWayIn,
    responses(
        (
            status = 200, description = "Stats inserted", body = Status,
            example = json!(Status::default())
        ),
        (
            status = 404, description = "User not found", body = Status,
            example = json!(Status{status: "User not found".to_string()})
        ),
        (
            status = 404, description = "Start auditory not found", body = Status,
            example = json!(Status{status: "Start auditory not found".to_string()})
        ),
        (
            status = 404, description = "End auditory not found", body = Status,
            example = json!(Status{status: "End auditory not found".to_string()})
        ),
        (
            status = 422, description = "Validation failed", body = Status,
            example = json!(Status{status: "The request body is invalid: ...".to_string()})
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        ),
    ),
    tag = "Stat"
)]
#[put("start-way")]
async fn stat_way(
    data: web::Json<StartWayIn>,
    db: web::Data<DatabaseConnection>,
) -> ApiResult<Status> {
    match data.validate() {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::UnprocessableData(e.to_string()))
    }?;
    user_id::Entity::filter(data.user_id.clone(), db.get_ref(), "User".to_string()).await?;
    aud::Entity::filter(
        data.start_id.to_string(),
        db.get_ref(),
        "Start auditory".to_string(),
    )
    .await?;
    aud::Entity::filter(
        data.end_id.to_string(),
        db.get_ref(),
        "End auditory".to_string(),
    )
    .await?;
    data.to_owned().into_active_model().insert(db.get_ref()).await.status_ok()
}
