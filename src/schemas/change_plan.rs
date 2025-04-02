use sea_orm::{
    EntityTrait, IntoActiveModel, QueryOrder,
    Select, QueryFilter, ColumnTrait,
    ActiveValue::Set
};
use actix_web::{body::BoxBody, Responder};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use validator::Validate;
use utoipa::ToSchema;
use crate::schemas::validators::PLAN_RE;
use crate::entity::change_plan;
use crate::traits::Paginate;
use crate::schemas::Filter;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Validate)]
pub struct ChangePlanIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "A-0")]
    #[validate(length(min = 3, max=4), regex(path = *PLAN_RE))]
    pub plan_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Validate)]
pub struct ChangePlanOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "A-0")]
    #[validate(length(min = 3, max=4), regex(path = *PLAN_RE))]
    pub plan_id: String,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub visit_date: NaiveDateTime,
}

impl Default for ChangePlanOut {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            plan_id: "A-0".into(),
            visit_date: chrono::Utc::now().naive_utc(),
        }
    }
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

impl Responder for ChangePlanOut {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
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

impl Paginate<'_, change_plan::Entity, change_plan::Model> for ChangePlanOut {
    fn get_query(filter: &Filter) -> Select<change_plan::Entity> {
        if let Some(user_id) = filter.user_id {
            change_plan::Entity::find()
                .filter(change_plan::Column::UserId.eq(user_id))
                .order_by_asc(change_plan::Column::Id)
        } else {
            change_plan::Entity::find()
                .order_by_asc(change_plan::Column::Id)
        }
    }
}
