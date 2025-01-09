use actix_web::{put, web};
use sea_orm::DatabaseConnection;
use entity::{plan, user_id};
use crate::errors::Result as ApiResult;
use crate::schemas::ChangePlanIn;
use crate::schemas::status::Status;
use crate::traits::{ConversionToStatusTrait, CreateFromScheme, FilterTrait};

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
    db: web::Data<DatabaseConnection>
) -> ApiResult<Status> {
    user_id::Entity::filter(data.user_id.clone(), db.get_ref(), "User".to_string()).await?;
    plan::Entity::filter(data.plan_id.to_string(), db.get_ref(), "Changed plan".to_string()).await?;
    data.create(db.get_ref()).await.status_ok()
}
