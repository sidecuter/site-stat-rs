use std::sync::Arc;

use crate::entity::user::{Column, Entity, Model};
use crate::errors::{ApiError, ApiResult};
use actix_web::{web, FromRequest, HttpRequest};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

// const JWT_SECRET: &str = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
const JWT_EXPIRATION_HOURS: u64 = 1;

pub fn hash_password(password: &str) -> ApiResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> ApiResult<bool> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn authenticate_user(
    db_conn: &DatabaseConnection,
    login: &str,
    password: &str,
) -> ApiResult<Option<Model>> {
    let user_option = Entity::find()
        .filter(Column::Login.eq(login))
        .one(db_conn)
        .await?;

    if let Some(db_user) = user_option {
        if verify_password(password, &db_user.hash)? {
            Ok(Some(db_user))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub fn create_token(user_id: i32) -> ApiResult<String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| ApiError::InternalError("Invalid time".to_string()))?
        .as_secs() as usize;
    let exp = now + (JWT_EXPIRATION_HOURS * 3600) as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat: now,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("asdasd".as_bytes()),
    )?;
    Ok(token)
}

pub struct CurrentUser(pub Model);

async fn get_current_user(db_conn: &DatabaseConnection, user_id: i32) -> ApiResult<Option<Model>> {
    Ok(Entity::find()
        .filter(Column::Id.eq(user_id).and(Column::IsActive.eq(true)))
        .one(db_conn)
        .await?)
}

async fn func_helper(
    auth_header_value: String,
    db_conn: Arc<DatabaseConnection>,
) -> ApiResult<CurrentUser> {
    let bearer_token: Vec<&str> = auth_header_value.split_whitespace().collect();
    if bearer_token[0] != "Bearer" {
        return Err(ApiError::InvalidToken);
    }

    let token = bearer_token[1];

    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("asdasd".as_bytes()),
        &validation,
    )
    .map_err(|_| ApiError::InvalidToken)?;

    let user_id: i32 = token_data
        .claims
        .sub
        .parse()
        .map_err(|_| ApiError::InvalidToken)?;

    match get_current_user(&db_conn, user_id).await? {
        Some(user) => Ok(CurrentUser(user)),
        None => Err(ApiError::UserInactive),
    }
}

impl FromRequest for CurrentUser {
    type Error = ApiError;
    type Future = std::pin::Pin<Box<dyn futures::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .and_then(|hv| hv.to_str().ok())
            .map(|s| s.to_string())
            .ok_or(ApiError::InvalidToken);

        let auth_header_value = match auth_header {
            Ok(val) => val,
            Err(e) => {
                return Box::pin(async move { Err(e) });
            }
        };

        let db_conn = req
            .app_data::<web::Data<DatabaseConnection>>()
            .cloned()
            .ok_or_else(|| ApiError::InternalError("No db pool available".to_string()));

        let db_conn = match db_conn {
            Ok(conn) => conn.into_inner(),
            Err(e) => {
                return Box::pin(async move { Err(e) });
            }
        };
        Box::pin(func_helper(auth_header_value, Arc::clone(&db_conn)))
    }
}
