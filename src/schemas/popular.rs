use sea_orm::FromQueryResult;
use crate::impl_responder;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Clone, ToSchema, FromQueryResult)]
pub struct PopularAud {
    #[sea_orm(alias = "Id")]
    pub id: String,
    #[sea_orm(alias = "Cnt")]
    pub cnt: i32
}

#[derive(Serialize, Clone, ToSchema)]
pub struct Popular(pub Vec<PopularAud>);

impl_responder!(Popular);
