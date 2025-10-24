use crate::config::AppConfig;
use crate::entity::{role_right_goal, user, user_role};
use crate::errors::{ApiError, ApiResult};
use crate::traits::EntityId;
use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ExprTrait, QueryFilter, QuerySelect, QueryTrait};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

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
) -> ApiResult<Option<user::Model>> {
    let user_option = user::Entity::find()
        .filter(user::Column::Login.eq(login))
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

pub fn create_token(user_id: i32, jwt_secret: &str) -> ApiResult<String> {
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
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;
    Ok(token)
}

pub struct CurrentUser(pub user::Model);

async fn get_current_user(
    db_conn: &DatabaseConnection,
    user_id: i32,
) -> ApiResult<Option<user::Model>> {
    Ok(user::Entity::find()
        .filter(
            user::Column::Id
                .eq(user_id)
                .and(user::Column::IsActive.eq(true)),
        )
        .one(db_conn)
        .await?)
}

fn validate_and_extract_token(
    token_str: String,
    jwt_secret: String,
) -> ApiResult<TokenData<Claims>> {
    let bearer_token: Vec<&str> = token_str.split_whitespace().collect();
    if bearer_token[0] != "Bearer" {
        return Err(ApiError::InvalidToken);
    }

    let token = bearer_token[1];

    let validation = Validation::default();
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|_| ApiError::InvalidToken)
}

async fn func_helper(
    auth_header_value: String,
    db_conn: Arc<DatabaseConnection>,
    jwt_secret: String,
) -> ApiResult<CurrentUser> {
    let token_data = validate_and_extract_token(auth_header_value, jwt_secret)?;

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

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let (auth_header, db_conn, jwt_secret) = match extractor(req) {
            Ok(v) => v,
            Err(e) => return Box::pin(async move { Err(e) }),
        };

        Box::pin(func_helper(auth_header, db_conn, jwt_secret))
    }
}

async fn gather_rights(
    db_conn: &DatabaseConnection,
    current_user_id: i32,
) -> ApiResult<Vec<role_right_goal::Model>> {
    let qr = user_role::Entity::find()
        .filter(user_role::Column::UserId.eq(current_user_id))
        .select_only()
        .column(user_role::Column::RoleId)
        .into_query();
    Ok(role_right_goal::Entity::find()
        .filter(role_right_goal::Column::RoleId.in_subquery(qr))
        .distinct_on([(
            role_right_goal::Column::RightId,
            role_right_goal::Column::GoalId,
        )])
        .all(db_conn)
        .await?)
}

fn extractor(req: &HttpRequest) -> ApiResult<(String, Arc<DatabaseConnection>, String)> {
    let auth_header = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|hv| hv.to_str().ok())
        .map(|s| s.to_string())
        .ok_or(ApiError::TokenNotPresent)?;

    let jwt_secret = req
        .app_data::<web::Data<AppConfig>>()
        .map(|v| v.jwt_secret.clone())
        .ok_or(ApiError::InternalError("Can't get jwt secret".to_string()))?;

    let db_conn = req
        .app_data::<web::Data<DatabaseConnection>>()
        .cloned()
        .ok_or_else(|| ApiError::InternalError("No db pool available".to_string()))?;

    Ok((auth_header, Arc::clone(&db_conn), jwt_secret))
}

async fn is_capable_helper(
    auth_header_value: String,
    db_conn: Arc<DatabaseConnection>,
    jwt_secret: String,
    right_id: i32,
    goal_id: i32,
) -> ApiResult<bool> {
    let token_data = validate_and_extract_token(auth_header_value, jwt_secret)?;

    let user_id: i32 = token_data
        .claims
        .sub
        .parse()
        .map_err(|_| ApiError::InvalidToken)?;

    let rights = gather_rights(&db_conn, user_id).await?;
    Ok(rights
        .into_iter()
        .any(|right_goal| (right_goal.right_id, right_goal.goal_id) == (right_id, goal_id)))
}

pub struct IsCapable<R, G>(PhantomData<R>, PhantomData<G>);

impl<R: EntityId, G: EntityId> FromRequest for IsCapable<R, G> {
    type Error = ApiError;
    type Future = std::pin::Pin<Box<dyn futures::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let (auth_header, db_conn, jwt_secret) = match extractor(req) {
            Ok(v) => v,
            Err(e) => return Box::pin(async move { Err(e) }),
        };

        Box::pin(async move {
            match is_capable_helper(auth_header, db_conn, jwt_secret, R::ID, G::ID).await {
                Ok(is_capable) if is_capable => Ok(Self(PhantomData::<R>, PhantomData::<G>)),
                Err(e) => Err(e),
                _ => Err(ApiError::NotAllowed(
                    "User can't execute this action".to_string(),
                )),
            }
        })
    }
}
