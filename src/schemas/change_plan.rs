use crate::entity::change_plan;
use crate::schemas::validators::PLAN_RE;
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
pub struct ChangePlanIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "A-0")]
    #[validate(length(min = 3, max=4), regex(path = *PLAN_RE))]
    pub plan_id: String,
}

#[derive(Serialize, ToSchema, Debug, Clone, Validate)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct ChangePlanOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "A-0")]
    #[validate(length(min = 3, max=4), regex(path = *PLAN_RE))]
    pub plan_id: String,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub visit_date: NaiveDateTime,
}

impl From<change_plan::Model> for ChangePlanOut {
    fn from(value: change_plan::Model) -> Self {
        Self {
            user_id: value.user_id,
            plan_id: value.plan_id,
            visit_date: value.visit_date,
        }
    }
}

impl IntoActiveModel<change_plan::ActiveModel> for ChangePlanIn {
    fn into_active_model(self) -> change_plan::ActiveModel {
        change_plan::ActiveModel {
            user_id: Set(self.user_id),
            visit_date: Set(chrono::Utc::now().naive_utc()),
            plan_id: Set(self.plan_id),
            ..Default::default()
        }
    }
}

impl_paginate!(ChangePlanOut, change_plan);
impl_responder!(ChangePlanOut);
