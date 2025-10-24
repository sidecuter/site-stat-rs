use crate::entity::select_aud;
use crate::schemas::validators::AUD_RE;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
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
