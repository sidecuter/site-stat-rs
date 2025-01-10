use std::env;
use std::future::Ready;
use std::time::Duration;
use actix_extensible_rate_limit::{
    backend::{
        memory::InMemoryBackend,
        SimpleInput, SimpleInputFunctionBuilder, SimpleOutput
    },
    HeaderCompatibleOutput, RateLimiter, RateLimiterBuilder
};
use actix_web::{dev::ServiceRequest, http::StatusCode, HttpResponse};
use crate::{
    errors::Error as ApiError,
    schemas::Status
};

fn rate_limit_exceeded(rate_info: &SimpleOutput) -> HttpResponse {
    let rest_time = rate_info.seconds_until_reset();
    let body = Status{status: ApiError::TooManyRequests(rest_time.to_string()).to_string()};

    HttpResponse::build(StatusCode::TOO_MANY_REQUESTS)
        .append_header(("x-rate-limit", rate_info.limit()))
        .append_header(("x-ratelimit-remaining", rate_info.remaining()))
        .append_header(("x-ratelimit-reset", rest_time))
        .json(body)
}

pub fn init_ip(
    backend: InMemoryBackend
) -> RateLimiterBuilder<
    InMemoryBackend,
    SimpleOutput,
    impl Fn(&ServiceRequest) -> Ready<Result<SimpleInput, actix_web::Error>>
> {
    let interval: u64 = env::var("RATE_LIMIT_INTERVAL")
        .unwrap_or("1".to_string())
        .parse()
        .expect("Invalid rate limit time in seconds");
    let max_request: u64 = env::var("RATE_LIMIT_MAX_REQUEST")
        .unwrap_or("1".to_string())
        .parse()
        .expect("Invalid rate limit max request count");
    let input = SimpleInputFunctionBuilder::new(
        Duration::from_secs(interval),
        max_request
    )
        .real_ip_key()
        .build();
    RateLimiter::builder(backend, input)
        .add_headers()
        .request_denied_response(rate_limit_exceeded)
}

pub fn create_in_memory_rate_limit() -> RateLimiter<
    InMemoryBackend,
    SimpleOutput,
    impl Fn(&ServiceRequest) -> Ready<Result<SimpleInput, actix_web::Error>>
> {
    let rl_backend = InMemoryBackend::builder().build();
    let rl_builder = init_ip(rl_backend.clone());
    rl_builder.build()
}
