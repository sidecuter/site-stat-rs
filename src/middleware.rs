use crate::{app_state::AppState, errors::Error as ApiError};
use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::Data,
    Error, Responder,
};

pub async fn api_key_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let state = req.app_data::<Data<AppState>>();
    let admin_key = if let Some(state_unwrap) = state {
        state_unwrap.admin_key.clone()
    } else {
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string()
    };
    if req.query_string().contains(&format!("api_key={admin_key}")) {
        next.call(req).await
    } else {
        let (request, _pl) = req.into_parts();
        let response = ApiError::NotAllowed("Specified api_key is not present in app".to_string())
            .respond_to(&request);
        Ok(ServiceResponse::new(request, response))
    }
}
