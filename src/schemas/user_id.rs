use crate::traits::CreateFromScheme;
use actix_web::body::BoxBody;
use actix_web::Responder;
use chrono::NaiveDateTime;
use crate::entity::user_id;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};
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

impl From<user_id::Model> for UserId {
    fn from(value: user_id::Model) -> Self {
        Self {
            user_id: value.user_id,
            creation_date: value.creation_date,
        }
    }
}

impl From<user_id::ActiveModel> for UserId {
    fn from(value: user_id::ActiveModel) -> Self {
        Self {
            user_id: value.user_id.unwrap(),
            creation_date: value.creation_date.unwrap(),
        }
    }
}

impl Responder for UserId {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}

impl CreateFromScheme<user_id::Model> for UserId {
    async fn create(&self, db: &DatabaseConnection) -> Result<user_id::Model, DbErr> {
        user_id::ActiveModel {
            user_id: ActiveValue::Set(self.user_id),
            creation_date: ActiveValue::Set(self.creation_date),
        }
        .insert(db)
        .await
    }
}
