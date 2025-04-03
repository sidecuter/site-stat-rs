use crate::errors::ApiResult;
use crate::schemas::Status;
use actix_web::get;

#[utoipa::path(
    get,
    path = "/v2/healthcheck",
    responses(
        (
            status = 200, description = "User id generated", body = Status,
            example = json!(Status{status: "Server is running".to_string()})
        )
    ),
    tag = "Healthcheck"
)]
#[get("/healthcheck")]
async fn get_status() -> ApiResult<Status> {
    Ok(Status{status: "Server is running".to_string()})
}
