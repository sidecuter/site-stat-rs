use crate::impl_responder;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Clone, ToSchema)]
pub struct Popular(pub Vec<String>);

impl_responder!(Popular);
