use crate::{config::AppConfig, errors::ApiError};
use actix_governor::governor::middleware::StateInformationMiddleware;
use actix_governor::{Governor, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::Data,
    Error, Responder,
};

#[allow(clippy::doc_markdown)]
/// Check for api key in request headers
///
/// # Errors
/// ApiKey in request don't match ApiKey in config
#[allow(clippy::future_not_send)]
pub async fn api_key_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    if let Some(config) = req.app_data::<Data<AppConfig>>() {
        if req
            .head()
            .headers()
            .get("Api-Key")
            .is_some_and(|hv| hv.as_bytes() == config.admin_key.as_bytes())
        {
            next.call(req).await
        } else {
            let (request, _pl) = req.into_parts();
            let response =
                ApiError::NotAllowed("Specified Api-Key is not present in app".to_string())
                    .respond_to(&request);
            Ok(ServiceResponse::new(request, response))
        }
    } else {
        next.call(req).await
    }
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn build_rate_limits() -> Governor<PeerIpKeyExtractor, StateInformationMiddleware> {
    let config = GovernorConfigBuilder::default()
        .burst_size(1)
        .requests_per_second(1)
        .use_headers()
        .finish()
        .unwrap();
    Governor::new(&config)
}
