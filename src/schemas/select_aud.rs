use crate::schemas::validators::AuditoryId;
use crate::traits::{impl_paginate_trait, CreateFromScheme};
use actix_web::body::BoxBody;
use actix_web::Responder;
use chrono::NaiveDateTime;
use entity::select_aud;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SelectAuditoryIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "a-100")]
    pub auditory_id: AuditoryId,
    #[schema(example = true)]
    pub success: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SelectAuditoryOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "a-100")]
    pub auditory_id: AuditoryId,
    #[schema(example = true)]
    pub success: bool,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub visit_date: NaiveDateTime,
}

impl Default for SelectAuditoryIn {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            auditory_id: "a-100".into(),
            success: true,
        }
    }
}

impl Default for SelectAuditoryOut {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            auditory_id: "a-100".into(),
            visit_date: chrono::Utc::now().naive_utc(),
            success: true,
        }
    }
}

impl From<select_aud::Model> for SelectAuditoryOut {
    fn from(value: select_aud::Model) -> Self {
        Self {
            user_id: value.user_id,
            auditory_id: value.auditory_id.into(),
            visit_date: value.visit_date,
            success: value.success,
        }
    }
}

impl From<select_aud::ActiveModel> for SelectAuditoryOut {
    fn from(value: select_aud::ActiveModel) -> Self {
        Self {
            user_id: value.user_id.unwrap(),
            auditory_id: value.auditory_id.unwrap().into(),
            visit_date: value.visit_date.unwrap(),
            success: value.success.unwrap(),
        }
    }
}

impl Responder for SelectAuditoryOut {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}

impl CreateFromScheme<select_aud::Model> for SelectAuditoryIn {
    async fn create(&self, db: &DatabaseConnection) -> Result<select_aud::Model, DbErr> {
        select_aud::ActiveModel {
            user_id: ActiveValue::Set(self.user_id),
            visit_date: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            auditory_id: ActiveValue::Set(self.auditory_id.to_string()),
            success: ActiveValue::Set(self.success),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}

impl_paginate_trait!(
    SelectAuditoryOut,
    entity::select_aud::Entity,
    entity::select_aud::Column::Id
);
