use crate::schemas::validators::AuditoryId;
use crate::traits::{impl_paginate_trait, CreateFromScheme};
use actix_web::body::BoxBody;
use actix_web::Responder;
use chrono::NaiveDateTime;
use entity::start_way;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct StartWayIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "a-100")]
    pub start_id: AuditoryId,
    #[schema(example = "a-100")]
    pub end_id: AuditoryId,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct StartWayOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "a-100")]
    pub start_id: AuditoryId,
    #[schema(example = "a-101")]
    pub end_id: AuditoryId,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub visit_date: NaiveDateTime,
}

impl Default for StartWayIn {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            start_id: "a-100".into(),
            end_id: "a-101".into(),
        }
    }
}

impl Default for StartWayOut {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            start_id: "a-100".into(),
            end_id: "a-101".into(),
            visit_date: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<start_way::Model> for StartWayOut {
    fn from(value: start_way::Model) -> Self {
        Self {
            user_id: value.user_id,
            start_id: value.start_id.into(),
            end_id: value.end_id.into(),
            visit_date: value.visit_date,
        }
    }
}

impl From<start_way::ActiveModel> for StartWayOut {
    fn from(value: start_way::ActiveModel) -> Self {
        Self {
            user_id: value.user_id.unwrap(),
            start_id: value.start_id.unwrap().into(),
            end_id: value.end_id.unwrap().into(),
            visit_date: value.visit_date.unwrap(),
        }
    }
}

impl Responder for StartWayOut {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}

impl CreateFromScheme<start_way::Model> for StartWayIn {
    async fn create(&self, db: &DatabaseConnection) -> Result<start_way::Model, DbErr> {
        start_way::ActiveModel {
            user_id: ActiveValue::Set(self.user_id),
            visit_date: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            start_id: ActiveValue::Set(self.start_id.to_string()),
            end_id: ActiveValue::Set(self.end_id.to_string()),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}

impl_paginate_trait!(
    StartWayOut,
    entity::start_way::Entity,
    entity::start_way::Column::Id
);
