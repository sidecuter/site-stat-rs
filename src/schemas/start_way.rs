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
use crate::schemas::validators::AUD_RE;
use crate::entity::start_way;
use crate::traits::Paginate;
use crate::schemas::Filter;

#[derive(Deserialize, ToSchema, Debug, Clone, Validate)]
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

impl Default for StartWayIn {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            start_id: "a-100".into(),
            end_id: "a-101".into(),
        }
    }
}

impl Default for StartWayOut {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            start_id: "a-100".into(),
            end_id: "a-101".into(),
            visit_date: chrono::Utc::now().naive_utc(),
        }
    }
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

impl From<start_way::ActiveModel> for StartWayOut {
    fn from(value: start_way::ActiveModel) -> Self {
        Self {
            user_id: value.user_id.unwrap(),
            start_id: value.start_id.unwrap(),
            end_id: value.end_id.unwrap(),
            visit_date: value.visit_date.unwrap(),
        }
    }
}

impl Responder for StartWayOut {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
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

impl Paginate<'_, start_way::Entity, start_way::Model> for StartWayOut {
    fn get_query(filter: &Filter) -> Select<start_way::Entity> {
        if let Some(user_id) = filter.user_id {
            start_way::Entity::find()
                .filter(start_way::Column::UserId.eq(user_id))
                .order_by_asc(start_way::Column::Id)
        } else {
            start_way::Entity::find()
                .order_by_asc(start_way::Column::UserId)
        }
    }
}
