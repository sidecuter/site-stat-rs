use actix_web::{get, web};
use sea_orm::DatabaseConnection;
use crate::errors::Result as ApiResult;
use crate::schemas::{user_id::UserId, status::Status, traits::OpenApiExample};
use crate::traits::{ConversionTrait, CreateFromScheme};

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
async fn get_user_id(
    db: web::Data<DatabaseConnection>
) -> ApiResult<UserId>{
    UserId::default().create(db.get_ref()).await.convert()
}