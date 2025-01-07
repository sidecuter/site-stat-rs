use actix_web::{error, http::{StatusCode, header::ContentType}, HttpResponse};
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
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::UnprocessableData(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
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

pub type Result<T> = std::result::Result<T, Error>;
