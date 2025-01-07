use actix_web::{put, web};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use entity::site_stat;
use crate::schemas::site_stat::{SiteStatisticsIn};
use crate::errors::{Result, ErrorTrait};
use crate::schemas::status::Status;

#[put("site")]
async fn stat_site(
    data: web::Json<SiteStatisticsIn>,
    db: web::Data<DatabaseConnection>
) -> Result<Status> {
    site_stat::ActiveModel{
        user_id: ActiveValue::Set(data.user_id),
        visit_date: ActiveValue::Set(chrono::offset::Utc::now().naive_utc()),
        endpoint: ActiveValue::Set(data.endpoint.clone()),
        ..Default::default()
    }.insert(db.get_ref()).await.map_err(|e| e.error()).map(|_| Status::default())
}