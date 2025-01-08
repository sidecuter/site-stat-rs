use actix_web::Responder;
use sea_orm::{DbErr, ModelTrait};
use crate::errors::Result as ApiResult;
use crate::schemas::pagination::Pagination;
use crate::schemas::status::Status;
use serde::Serialize;

pub trait ConversionTrait<T> {
    type Output;

    fn convert(self) -> Self::Output;
}

pub trait ConversionToStatusTrait<T> {
    type Output;

    fn status_ok(self) -> Self::Output;
}

pub trait ConversionToPaginationTrait<T> {
    type Output;

    fn to_response(self) -> Self::Output;
}

impl<T: Responder + From<W>, W: ModelTrait> ConversionTrait<T> for Result<W, DbErr> {
    type Output = ApiResult<T>;

    fn convert(self) -> Self::Output {
        self.map_err(|e| e.into()).map(|v| v.into())
    }
}

impl<T: Responder + From<Status>, W: ModelTrait> ConversionToStatusTrait<T> for Result<W, DbErr> {
    type Output = ApiResult<T>;
    fn status_ok(self) -> Self::Output {
        self.map_err(|e| e.into()).map(|_| Status::default().into())
    }
}

impl<T: Responder + Serialize + Clone> ConversionToPaginationTrait<T> for Result<Pagination<T>, DbErr> {
    type Output = ApiResult<Pagination<T>>;

    fn to_response(self) -> Self::Output {
        self.map_err(|e| e.into())
    }
}

