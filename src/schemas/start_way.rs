use crate::entity::start_way;
use crate::schemas::validators::AUD_RE;
use crate::schemas::Filter;
use crate::traits::Paginate;
use crate::{impl_paginate, impl_responder};
use chrono::NaiveDateTime;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Select,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Debug, Clone, Validate)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct StartWayIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "a-100")]
    #[validate(length(min = 3), regex(path = *AUD_RE))]
    pub start_id: String,
    #[schema(example = "a-100")]
    #[validate(length(min = 3), regex(path = *AUD_RE))]
    pub end_id: String,
}

#[derive(Serialize, ToSchema, Debug, Clone, Validate)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct StartWayOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "a-100")]
    #[validate(length(min = 3), regex(path = *AUD_RE))]
    pub start_id: String,
    #[schema(example = "a-101")]
    #[validate(length(min = 3), regex(path = *AUD_RE))]
    pub end_id: String,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub visit_date: NaiveDateTime,
}

impl From<start_way::Model> for StartWayOut {
    fn from(value: start_way::Model) -> Self {
        Self {
            user_id: value.user_id,
            start_id: value.start_id,
            end_id: value.end_id,
            visit_date: value.visit_date,
        }
    }
}

impl IntoActiveModel<start_way::ActiveModel> for StartWayIn {
    fn into_active_model(self) -> start_way::ActiveModel {
        start_way::ActiveModel {
            user_id: Set(self.user_id),
            start_id: Set(self.start_id),
            end_id: Set(self.end_id),
            visit_date: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
    }
}

impl_paginate!(StartWayOut, start_way);
impl_responder!(StartWayOut);
