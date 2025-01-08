use actix_web::{get, web};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use entity::user_id;
use crate::errors::Result as ApiResult;
use crate::schemas::{user_id::UserId, status::Status, traits::OpenApiExample};
use crate::traits::ConversionTrait;

#[utoipa::path(
    get,
    path = "/api/get/user-id",
    request_body = UserId,
    responses(
        (
            status = 200, description = "User id generated", body = UserId,
            example = json!(UserId::openapi_example())
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        )
    ),
    tag = "Get"
)]
#[get("/user-id")]
pub async fn get_user_id(
    db: web::Data<DatabaseConnection>
) -> ApiResult<UserId>{
    let default_user = UserId::default();
    let active_model = user_id::ActiveModel {
        user_id: ActiveValue::Set(default_user.user_id),
        creation_date: ActiveValue::Set(default_user.creation_date)
    };
    active_model.insert(db.get_ref()).await.convert()
}