use crate::entity::{plan, user_id};
use crate::errors::{ApiError, ApiResult};
use crate::schemas::{ChangePlanIn, Status};
use crate::traits::{ConversionToStatusTrait, FilterTrait};
use actix_web::{put, web};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};
use validator::Validate;

#[utoipa::path(
    put,
    path = "/api/stat/change-plan",
    request_body = ChangePlanIn,
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
            status = 404, description = "Changed plan not found", body = Status,
            example = json!(Status{status: "Changed plan not found".to_string()})
        ),
        (
            status = 422, description = "Validation failed", body = Status,
            example = json!(Status{status: "The request body is invalid: ...".to_string()})
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        )
    ),
    tag = "Stat"
)]
#[put("change-plan")]
async fn stat_plan(
    data: web::Json<ChangePlanIn>,
    db: web::Data<DatabaseConnection>,
) -> ApiResult<Status> {
    match data.validate() {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::UnprocessableData(e.to_string())),
    }?;
    user_id::Entity::filter(data.user_id, db.get_ref(), "User".to_string()).await?;
    plan::Entity::filter(
        data.plan_id.clone(),
        db.get_ref(),
        "Changed plan".to_string(),
    )
    .await?;
    data.to_owned()
        .into_active_model()
        .insert(db.get_ref())
        .await
        .status_ok()
}
