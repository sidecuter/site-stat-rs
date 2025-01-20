use crate::{
    errors::Result as ApiResult,
    schemas::Status,
};
use actix_web::Responder;
use sea_orm::{DbErr, ModelTrait};

pub trait ConversionTrait<T> {
    type Output;

    fn convert(self) -> Self::Output;
}

pub trait ConversionToStatusTrait<T> {
    type Output;

    fn status_ok(self) -> Self::Output;
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
