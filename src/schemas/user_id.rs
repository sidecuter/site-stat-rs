use sea_orm::{IntoActiveModel, ActiveValue::Set};
use actix_web::{body::BoxBody, Responder};
use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;
use crate::entity::user_id;
use crate::impl_responder;

#[derive(ToSchema, Debug, Serialize, Clone)]
pub struct UserId {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub creation_date: NaiveDateTime,
}

impl Default for UserId {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            creation_date: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<user_id::Model> for UserId {
    fn from(value: user_id::Model) -> Self {
        Self {
            user_id: value.user_id,
            creation_date: value.creation_date,
        }
    }
}

impl IntoActiveModel<user_id::ActiveModel> for UserId {
    fn into_active_model(self) -> user_id::ActiveModel {
        user_id::ActiveModel {
            user_id: Set(self.user_id),
            creation_date: Set(self.creation_date),
        }
    }
}

impl_responder!(UserId);
