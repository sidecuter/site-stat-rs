use sea_orm::{
    EntityTrait, IntoActiveModel, QueryOrder,
    Select, QueryFilter, ColumnTrait,
    ActiveValue::Set
};
use actix_web::{body::BoxBody, Responder};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use utoipa::ToSchema;
use crate::entity::site_stat;
use crate::traits::Paginate;
use crate::schemas::Filter;

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

impl IntoActiveModel<site_stat::ActiveModel> for SiteStatisticsIn {
    fn into_active_model(self) -> site_stat::ActiveModel {
        site_stat::ActiveModel {
            user_id: Set(self.user_id),
            visit_date: Set(chrono::Utc::now().naive_utc()),
            endpoint: Set(self.endpoint),
            ..Default::default()
        }
    }
}

impl Paginate<'_, site_stat::Entity, site_stat::Model> for SiteStatisticsOut {
    fn get_query(filter: &Filter) -> Select<site_stat::Entity> {
        if let Some(user_id) = filter.user_id {
            site_stat::Entity::find()
                .filter(site_stat::Column::UserId.eq(user_id))
                .order_by_asc(site_stat::Column::Id)
        } else {
            site_stat::Entity::find()
                .order_by_asc(site_stat::Column::UserId)
        }
    }
}
