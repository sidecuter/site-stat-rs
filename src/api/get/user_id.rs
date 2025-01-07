use actix_web::{get, web};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use entity::user_id;
use uuid::Uuid;
use chrono;
use crate::errors::{Error, ErrorTrait};
use crate::schemas::{user_id::UserId, status::Status, traits::OpenApiExample};

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
    pool: web::Data<DatabaseConnection>
) -> Result<UserId, Error>{
    user_id::ActiveModel {
        user_id: ActiveValue::Set(Uuid::new_v4()),
        creation_date: ActiveValue::Set(chrono::offset::Utc::now().naive_utc())
    }.insert(pool.get_ref()).await.map_err(|e| e.error()).map(|v| v.into())
}