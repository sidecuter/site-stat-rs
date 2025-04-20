use crate::schemas::Status;
use actix_web::error::BlockingError;
use actix_web::{
    body::BoxBody,
    error::{JsonPayloadError, QueryPayloadError},
    http::{header::ContentType, StatusCode},
    HttpRequest, HttpResponse, Responder, ResponseError,
};
use sea_orm::DbErr;
use std::sync::PoisonError;

#[derive(Debug, thiserror::Error, Clone)]
pub enum ApiError {
    #[error("{0}")]
    InternalError(String),
    #[error("{0}")]
    UnprocessableData(String),
    #[error("{0} not found")]
    NotFound(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    PathNotFound(String),
    #[error("{0}")]
    NotAllowed(String),
    #[error("Too many requests, retry in {0}s")]
    TooManyRequests(String),
    #[error("{0}")]
    UnsupportedMediaType(String),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnprocessableData(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::PathNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::NotAllowed(_) => StatusCode::FORBIDDEN,
            ApiError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            ApiError::UnsupportedMediaType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(Status {
                status: self.to_string(),
            })
    }
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        tracing::error!("{:?}", value.sql_err());
        let message = value.to_string();
        if message.contains("FOREIGN KEY") {
            Self::NotFound("External id".to_string())
        } else {
            Self::InternalError(value.to_string())
        }
    }
}

impl From<uuid::Error> for ApiError {
    fn from(value: uuid::Error) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl From<QueryPayloadError> for ApiError {
    fn from(err: QueryPayloadError) -> Self {
        match err {
            QueryPayloadError::Deserialize(err) => Self::UnprocessableData(err.to_string()),
            _ => Self::UnprocessableData("The parameters query are invalid".to_string()),
        }
    }
}

impl From<JsonPayloadError> for ApiError {
    fn from(err: JsonPayloadError) -> Self {
        match err {
            JsonPayloadError::ContentType => {
                Self::BadRequest("The content type is not `application/json`".to_string())
            }
            JsonPayloadError::Deserialize(err) => {
                Self::UnprocessableData(format!("The request body is invalid: {err}"))
            }
            _ => Self::BadRequest("The request body is invalid".to_string()),
        }
    }
}

impl From<BlockingError> for ApiError {
    fn from(value: BlockingError) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl<T> From<PoisonError<T>> for ApiError {
    fn from(value: PoisonError<T>) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl Responder for ApiError {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        self.error_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
