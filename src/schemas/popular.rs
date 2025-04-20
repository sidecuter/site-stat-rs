use crate::impl_responder;
use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Clone, ToSchema)]
pub struct Popular(pub Vec<String>);

impl_responder!(Popular);
