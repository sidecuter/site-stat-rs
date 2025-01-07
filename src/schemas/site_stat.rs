use actix_web::body::BoxBody;
use actix_web::Responder;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SiteStatisticsIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "/app")]
    pub endpoint: Option<String>
}

#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct SiteStatisticsOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "/app")]
    pub endpoint: Option<String>,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub visit_date: NaiveDateTime
}

impl Default for SiteStatisticsIn {
    fn default() -> Self {
        Self{
            user_id: uuid::Uuid::new_v4(),
            endpoint: Some("/app".to_string())
        }
    }
}

impl Default for SiteStatisticsOut {
    fn default() -> Self {
        Self{
            user_id: uuid::Uuid::new_v4(),
            endpoint: Some("/app".to_string()),
            visit_date: chrono::offset::Utc::now().naive_utc()
        }
    }
}

impl From<entity::site_stat::Model> for SiteStatisticsOut {
    fn from(value: entity::site_stat::Model) -> Self {
        Self {
            user_id: value.user_id,
            endpoint: value.endpoint,
            visit_date: value.visit_date
        }
    }
}

impl From<entity::site_stat::ActiveModel> for SiteStatisticsOut {
    fn from(value: entity::site_stat::ActiveModel) -> Self {
        Self {
            user_id: value.user_id.unwrap(),
            endpoint: value.endpoint.unwrap(),
            visit_date: value.visit_date.unwrap()
        }
    }
}

impl Responder for SiteStatisticsOut {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}