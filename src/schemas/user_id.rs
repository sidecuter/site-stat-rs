use crate::entity::user_ids;
use crate::impl_responder;
use chrono::NaiveDateTime;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Serialize;
use utoipa::ToSchema;

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

impl From<user_ids::Model> for UserId {
    fn from(value: user_ids::Model) -> Self {
        Self {
            user_id: value.user_id,
            creation_date: value.creation_date,
        }
    }
}

impl IntoActiveModel<user_ids::ActiveModel> for UserId {
    fn into_active_model(self) -> user_ids::ActiveModel {
        user_ids::ActiveModel {
            user_id: Set(self.user_id),
            creation_date: Set(self.creation_date),
        }
    }
}

impl_responder!(UserId);
