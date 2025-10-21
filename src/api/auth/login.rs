use crate::auth::{authenticate_user, create_token};
use crate::errors::{ApiError, ApiResult};
use crate::schemas::{LoginRequest, TokenResponse};
use actix_web::{post, web};
use sea_orm::DatabaseConnection;
use crate::config::AppConfig;

#[utoipa::path(post, path = "/api/auth/token", tag = "Auth")]
#[post("/token")]
async fn login(
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
