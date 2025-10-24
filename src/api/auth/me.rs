use actix_web::get;

use crate::{auth::CurrentUser, errors::ApiResult, schemas::UserResp};

#[get("/me")]
async fn me(user: CurrentUser) -> ApiResult<UserResp> {
    Ok(UserResp {
        login: user.0.login,
        is_active: user.0.is_active,
    })
}
