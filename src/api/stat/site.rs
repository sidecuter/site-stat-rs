use crate::entity::user_id;
use crate::errors::ApiResult;
use crate::schemas::{SiteStatisticsIn, Status};
use crate::traits::{ConversionToStatusTrait, FilterTrait};
use actix_web::{put, web};
use sea_orm::{DatabaseConnection, IntoActiveModel, ActiveModelTrait};

#[utoipa::path(
    put,
    path = "/api/stat/site",
    request_body = SiteStatisticsIn,
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
#[put("site")]
async fn stat_site(
    data: web::Json<SiteStatisticsIn>,
    db: web::Data<DatabaseConnection>,
) -> ApiResult<Status> {
    user_id::Entity::filter(data.user_id, db.get_ref(), "User".to_string()).await?;
    data.to_owned().into_active_model().insert(db.get_ref()).await.status_ok()
}
