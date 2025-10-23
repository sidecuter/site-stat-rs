use crate::auth::{authenticate_user, create_token};
use crate::errors::{ApiError, ApiResult};
use crate::schemas::{LoginRequest, TokenResponse, Status};
use actix_web::{post, web};
use sea_orm::DatabaseConnection;
use crate::config::AppConfig;

#[utoipa::path(
    post,
    path = "/api/auth/token",
    request_body(content = LoginRequest, content_type = "application/x-www-form-urlencoded"),
    responses(
        (
            status = 200, description = "JWT Token", body = TokenResponse,
            example = json!({
                "access_token": "<token>",
                "token_type": "bearer"
            })
        ),
        (
            status = 401, description = "Invalid credentials", body = Status,
            example = json!(Status{status: "Invalid credentials".to_string()})
        ),
        (
            status = 500, description = "Internal errors", body = Status,
            example = json!(Status{status: "Internal error".to_string()})
        ),
    ),
    tag = "Auth")]
#[post("/token")]
async fn token(
    login_data: web::Form<LoginRequest>,
    db_conn: web::Data<DatabaseConnection>,
    config: web::Data<AppConfig>
) -> ApiResult<TokenResponse> {
    let user = authenticate_user(&db_conn, &login_data.username, &login_data.password).await?;

    if let Some(user) = user {
        let token = create_token(user.id, &config.jwt_token)?;
        Ok(TokenResponse {
            access_token: token,
            token_type: "bearer".to_string(),
        })
    } else {
        Err(ApiError::InvalidCredentials)
    }
}
