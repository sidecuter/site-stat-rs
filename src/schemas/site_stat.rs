use crate::traits::{impl_paginate_trait, CreateFromScheme};
use actix_web::body::BoxBody;
use actix_web::Responder;
use chrono::NaiveDateTime;
use entity::site_stat;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SiteStatisticsIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "/app")]
    pub endpoint: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SiteStatisticsOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "/app")]
    pub endpoint: Option<String>,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub visit_date: NaiveDateTime,
}

impl Default for SiteStatisticsIn {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            endpoint: Some("/app".to_string()),
        }
    }
}

impl Default for SiteStatisticsOut {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            endpoint: Some("/app".to_string()),
            visit_date: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<site_stat::Model> for SiteStatisticsOut {
    fn from(value: site_stat::Model) -> Self {
        Self {
            user_id: value.user_id,
            endpoint: value.endpoint,
            visit_date: value.visit_date,
        }
    }
}

impl From<site_stat::ActiveModel> for SiteStatisticsOut {
    fn from(value: site_stat::ActiveModel) -> Self {
        Self {
            user_id: value.user_id.unwrap(),
            endpoint: value.endpoint.unwrap(),
            visit_date: value.visit_date.unwrap(),
        }
    }
}

impl Responder for SiteStatisticsOut {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}

impl CreateFromScheme<site_stat::Model> for SiteStatisticsIn {
    async fn create(&self, db: &DatabaseConnection) -> Result<site_stat::Model, DbErr> {
        site_stat::ActiveModel {
            user_id: ActiveValue::Set(self.user_id),
            visit_date: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            endpoint: ActiveValue::Set(self.endpoint.clone()),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}

impl_paginate_trait!(
    SiteStatisticsOut,
    entity::site_stat::Entity,
    entity::site_stat::Column::Id
);
