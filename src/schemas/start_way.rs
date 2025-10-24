use crate::entity::start_way;
use crate::schemas::validators::AUD_RE;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
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
