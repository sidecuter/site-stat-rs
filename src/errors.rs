use actix_web::{http::{StatusCode, header::ContentType}, body::BoxBody,
                HttpRequest, HttpResponse, Responder, ResponseError, Error as ActixError};
use actix_web::error::{JsonPayloadError, QueryPayloadError};
use log::{log, Level};
use sea_orm::DbErr;
use crate::schemas::status;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
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
    NotAllowed(String)
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::UnprocessableData(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::PathNotFound(_) => StatusCode::NOT_FOUND,
            Error::NotAllowed(_) => StatusCode::FORBIDDEN
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(status::Status{status: self.to_string()})
    }
}

impl From<DbErr> for Error {
    fn from(value: DbErr) -> Self {
        log!(Level::Error, "{:?}", value.sql_err());
        let message = value.to_string();
        if message.contains("FOREIGN KEY") {
            Self::NotFound("External id".to_string())
        } else {
            Self::InternalError(value.to_string())
        }
    }
}

pub trait ErrorTrait {
    fn error(self) -> Error;
}

impl ErrorTrait for DbErr {
    fn error(self) -> Error {
        self.into()
    }
}

impl From<QueryPayloadError> for Error {
    fn from(err: QueryPayloadError) -> Self {
        match err {
            QueryPayloadError::Deserialize(err) => Self::UnprocessableData(err.to_string()),
            _ => Self::UnprocessableData("The parameters query are invalid".to_string()),
        }
    }
}

impl From<JsonPayloadError> for Error {
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

impl Responder for Error {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        self.error_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
