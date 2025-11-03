use actix_web::{body::BoxBody, Responder};
use sea_orm::FromQueryResult;
use serde::Serialize;
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResp {
    pub(crate) login: String,
    pub(crate) is_active: bool,
    pub(crate) rights_by_goals: HashMap<String, Vec<String>>,
}

impl Responder for UserResp {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}

#[derive(Debug, FromQueryResult)]
pub struct RightsGoals {
    pub(crate) right_id: i32,
    pub(crate) right_name: String,
    pub(crate) goal_id: i32,
    pub(crate) goal_name: String,
}
