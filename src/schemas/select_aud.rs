use actix_web::body::BoxBody;
use actix_web::Responder;
use chrono::NaiveDateTime;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use entity::select_aud;
use crate::traits::CreateFromScheme;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SelectAuditoryIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "a-100")]
    pub auditory_id: String
}

#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct SelectAuditoryOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "a-100")]
    pub auditory_id: String,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub visit_date: NaiveDateTime
}

impl Default for SelectAuditoryIn {
    fn default() -> Self {
        Self{
            user_id: uuid::Uuid::new_v4(),
            auditory_id: "a-100".to_string()
        }
    }
}

impl Default for SelectAuditoryOut {
    fn default() -> Self {
        Self{
            user_id: uuid::Uuid::new_v4(),
            auditory_id: "a-100".to_string(),
            visit_date: chrono::offset::Utc::now().naive_utc()
        }
    }
}

impl From<select_aud::Model> for SelectAuditoryOut {
    fn from(value: select_aud::Model) -> Self {
        Self {
            user_id: value.user_id,
            auditory_id: value.auditory_id,
            visit_date: value.visit_date
        }
    }
}

impl From<select_aud::ActiveModel> for SelectAuditoryOut {
    fn from(value: select_aud::ActiveModel) -> Self {
        Self {
            user_id: value.user_id.unwrap(),
            auditory_id: value.auditory_id.unwrap(),
            visit_date: value.visit_date.unwrap()
        }
    }
}

impl Responder for SelectAuditoryOut {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}

impl CreateFromScheme<select_aud::Model> for SelectAuditoryOut {
    async fn create(&self, db: &DatabaseConnection) -> Result<select_aud::Model, DbErr> {
        select_aud::ActiveModel {
            user_id: ActiveValue::Set(self.user_id),
            visit_date: ActiveValue::Set(chrono::offset::Utc::now().naive_utc()),
            auditory_id: ActiveValue::Set(self.auditory_id.clone()),
            ..Default::default()
        }.insert(db).await
    }
}
