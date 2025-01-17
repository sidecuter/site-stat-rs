use crate::entity::change_plan;
use crate::schemas::validators::PlanId;
use crate::traits::{impl_paginate_trait, CreateFromScheme};
use actix_web::body::BoxBody;
use actix_web::Responder;
use chrono::NaiveDateTime;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ChangePlanIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "A-0")]
    pub plan_id: PlanId,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ChangePlanOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "A-0")]
    pub plan_id: PlanId,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub visit_date: NaiveDateTime,
}

impl Default for ChangePlanIn {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            plan_id: "A-0".into(),
        }
    }
}

impl Default for ChangePlanOut {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            plan_id: "A-0".into(),
            visit_date: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<change_plan::Model> for ChangePlanOut {
    fn from(value: change_plan::Model) -> Self {
        Self {
            user_id: value.user_id,
            plan_id: value.plan_id.into(),
            visit_date: value.visit_date,
        }
    }
}

impl From<change_plan::ActiveModel> for ChangePlanOut {
    fn from(value: change_plan::ActiveModel) -> Self {
        Self {
            user_id: value.user_id.unwrap(),
            plan_id: value.plan_id.unwrap().into(),
            visit_date: value.visit_date.unwrap(),
        }
    }
}

impl Responder for ChangePlanOut {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}

impl CreateFromScheme<change_plan::Model> for ChangePlanIn {
    async fn create(&self, db: &DatabaseConnection) -> Result<change_plan::Model, DbErr> {
        change_plan::ActiveModel {
            user_id: ActiveValue::Set(self.user_id),
            visit_date: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            plan_id: ActiveValue::Set(self.plan_id.to_string()),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}

impl_paginate_trait!(
    ChangePlanOut,
    crate::entity::change_plan::Entity,
    crate::entity::change_plan::Column::Id
);
