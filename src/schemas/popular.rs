use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::Serialize;
use utoipa::ToSchema;
use crate::impl_responder;

#[derive(Serialize, Clone, ToSchema)]
pub struct Popular(pub Vec<String>);

impl_responder!(Popular);
