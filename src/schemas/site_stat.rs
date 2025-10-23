use crate::entity::site_stat;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct SiteStatisticsIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    #[schema(example = "/app")]
    pub endpoint: Option<String>,
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
