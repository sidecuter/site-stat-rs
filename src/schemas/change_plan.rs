use crate::entity::change_plan;
use crate::schemas::validators::PLAN_RE;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
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
