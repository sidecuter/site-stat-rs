use actix_web::{put, web};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use entity::{site_stat, user_id};
use crate::schemas::site_stat::{SiteStatisticsIn};
use crate::errors::Result as ApiResult;
use crate::schemas::status::Status;
use crate::traits::{ConversionToStatusTrait, FilterTrait};

#[utoipa::path(
    put,
    path = "/api/stat/site",
    request_body = SiteStatisticsIn,
    responses(
        (
            status = 200, description = "Stats inserted", body = Status,
            example = json!(Status::default())
        ),
        (
            status = 404, description = "User not found", body = Status,
            example = json!(Status{status: "User not found".to_string()})
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        )
    ),
    tag = "Stat"
)]
#[put("site")]
async fn stat_site(
    data: web::Json<SiteStatisticsIn>,
    db: web::Data<DatabaseConnection>
) -> ApiResult<Status> {
    user_id::Entity::filter(data.user_id.clone(), db.get_ref(), "User".to_string()).await?;
    let active_model = site_stat::ActiveModel {
        user_id: ActiveValue::Set(data.user_id),
        visit_date: ActiveValue::Set(chrono::offset::Utc::now().naive_utc()),
        endpoint: ActiveValue::Set(data.endpoint.clone()),
        ..Default::default()
    };
    active_model.insert(db.get_ref()).await.status_ok()
}