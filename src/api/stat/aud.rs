use crate::entity::{aud, user_id};
use crate::errors::Result as ApiResult;
use crate::schemas::{SelectAuditoryIn, Status};
use crate::traits::{ConversionToStatusTrait, FilterTrait};
use actix_web::{put, web};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};
use crate::middleware::build_rate_limits;

#[utoipa::path(
    put,
    path = "/api/stat/select-aud",
    request_body = SelectAuditoryIn,
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
            status = 404, description = "Auditory not found", body = Status,
            example = json!(Status{status: "Auditory not found".to_string()})
        ),
        (
            status = 422, description = "Validation failed", body = Status,
            example = json!(Status{status: "The request body is invalid: ...".to_string()})
        ),
        // (
        //     status = 429, description = "Too many requests", body = Status,
        //     example = json!(Status{status: "Too many requests, retry in 1s".to_string()})
        // ),CreateFromScheme
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        ),
    ),
    tag = "Stat"
)]
#[put("select-aud", wrap="build_rate_limits()")]
async fn stat_aud(
    data: web::Json<SelectAuditoryIn>,
    db: web::Data<DatabaseConnection>,
) -> ApiResult<Status> {
    user_id::Entity::filter(data.user_id.clone(), db.get_ref(), "User".to_string()).await?;
    aud::Entity::filter(
        data.auditory_id.to_string(),
        db.get_ref(),
        "Auditory".to_string(),
    )
    .await?;
    data.to_owned().into_active_model().insert(db.get_ref()).await.status_ok()
}
