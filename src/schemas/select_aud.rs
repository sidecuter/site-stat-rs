use crate::entity::select_aud;
use crate::schemas::validators::AUD_RE;
use crate::schemas::Filter;
use crate::traits::Paginate;
use crate::{impl_paginate, impl_responder};
use actix_web::{body::BoxBody, Responder};
use chrono::NaiveDateTime;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Select,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Debug, Clone, Validate)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct SelectAuditoryIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "a-100")]
    #[validate(length(min = 3), regex(path = *AUD_RE))]
    pub auditory_id: String,
    #[schema(example = true)]
    pub success: bool,
}

#[derive(Serialize, ToSchema, Debug, Clone, Validate)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct SelectAuditoryOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "a-100")]
    #[validate(length(min = 3), regex(path = *AUD_RE))]
    pub auditory_id: String,
    #[schema(example = true)]
    pub success: bool,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub visit_date: NaiveDateTime,
}

impl From<select_aud::Model> for SelectAuditoryOut {
    fn from(value: select_aud::Model) -> Self {
        Self {
            user_id: value.user_id,
            auditory_id: value.auditory_id,
            visit_date: value.visit_date,
            success: value.success,
        }
    }
}

impl IntoActiveModel<select_aud::ActiveModel> for SelectAuditoryIn {
    fn into_active_model(self) -> select_aud::ActiveModel {
        select_aud::ActiveModel {
            user_id: Set(self.user_id),
            visit_date: Set(chrono::Utc::now().naive_utc()),
            auditory_id: Set(self.auditory_id),
            success: Set(self.success),
            ..Default::default()
        }
    }
}

impl_paginate!(SelectAuditoryOut, select_aud);
impl_responder!(SelectAuditoryOut);
