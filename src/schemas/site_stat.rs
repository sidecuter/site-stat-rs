use crate::entity::site_stat;
use crate::schemas::Filter;
use crate::traits::Paginate;
use crate::{impl_paginate, impl_responder};
use chrono::NaiveDateTime;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Select,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct SiteStatisticsIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "/app")]
    pub endpoint: Option<String>,
}

#[derive(Serialize, ToSchema, Debug, Clone)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct SiteStatisticsOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "/app")]
    pub endpoint: Option<String>,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub visit_date: NaiveDateTime,
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

impl_paginate!(SiteStatisticsOut, site_stat);
impl_responder!(SiteStatisticsOut);
