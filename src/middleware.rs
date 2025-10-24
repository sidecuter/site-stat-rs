use actix_governor::governor::middleware::StateInformationMiddleware;
use actix_governor::{Governor, GovernorConfigBuilder, PeerIpKeyExtractor};

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
