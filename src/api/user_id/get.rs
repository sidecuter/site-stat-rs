use crate::errors::ApiResult;
use crate::schemas::{Status, UserId};
use crate::traits::ConversionTrait;
use actix_web::{get, web};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};

#[utoipa::path(
    get,
    path = "/v2/user-id/get",
    responses(
        (
            status = 200, description = "User id generated", body = UserId
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        )
    ),
    tag = "UserId"
)]
#[get("/get")]
async fn get_user_id(db: web::Data<DatabaseConnection>) -> ApiResult<UserId> {
    UserId::default().into_active_model().insert(db.get_ref()).await.convert()
}
