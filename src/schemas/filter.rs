use serde::{Deserialize, Serialize};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ModelTrait, PaginatorTrait, QueryOrder};
use utoipa::ToSchema;
use crate::schemas::validators::{ApiKey, Page, Size};
use crate::schemas::pagination::Pagination;
use crate::schemas::select_aud::SelectAuditoryOut;
use crate::schemas::site_stat::SiteStatisticsOut;
use crate::schemas::start_way::StartWayOut;
use crate::traits::Paginate;

#[derive(Deserialize, Clone, ToSchema)]
pub struct Filter{
    #[allow(dead_code)]
    #[schema(pattern = r"^[0-9a-f]{64}$", example = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")]
    api_key: ApiKey,
    user_id: Option<uuid::Uuid>,
    #[schema(example = 1, minimum = 1)]
    #[serde(default)]
    page: Page,
    #[schema(example = 50, maximum = 100)]
    #[serde(default)]
    size: Size
}

impl Paginate<Pagination<SiteStatisticsOut>> for Filter {
    async fn pagination(&self, db: &DatabaseConnection) -> Result<Pagination<SiteStatisticsOut>, DbErr> {
        let pages = if let Some(user_id) = self.user_id {
            let user_id = entity::prelude::UserId::find_by_id(user_id).one(db).await?;
            let user_id = user_id.unwrap();
            user_id.find_related(entity::prelude::SiteStat).order_by_asc(entity::site_stat::Column::Id)
                .paginate(db, self.size.get())
        } else {
            entity::site_stat::Entity::find().order_by_asc(entity::site_stat::Column::Id)
                .paginate(db, self.size.get())
        };
        let total_items = pages.num_items().await?;
        let all_pages = pages.num_pages().await?;
        let items = pages.fetch_page(self.page.get()-1).await?;
        Ok(return_answer::<SiteStatisticsOut>(
            items.into_iter().map(|model| model.into()).collect(),
            self, total_items, all_pages
        ))
    }
}

impl Paginate<Pagination<SelectAuditoryOut>> for Filter {
    async fn pagination(&self, db: &DatabaseConnection) -> Result<Pagination<SelectAuditoryOut>, DbErr> {
        let pages = if let Some(user_id) = self.user_id {
            let user_id = entity::prelude::UserId::find_by_id(user_id).one(db).await?;
            let user_id = user_id.unwrap();
            user_id.find_related(entity::prelude::SelectAud).order_by_asc(entity::select_aud::Column::Id)
                .paginate(db, self.size.get())
        } else {
            entity::select_aud::Entity::find().order_by_asc(entity::select_aud::Column::Id)
                .paginate(db, self.size.get())
        };
        let total_items = pages.num_items().await?;
        let all_pages = pages.num_pages().await?;
        let items = pages.fetch_page(self.page.get()-1).await?;
        Ok(return_answer::<SelectAuditoryOut>(
            items.into_iter().map(|model| model.into()).collect(),
            self, total_items, all_pages
        ))
    }
}

impl Paginate<Pagination<StartWayOut>> for Filter {
    async fn pagination(&self, db: &DatabaseConnection) -> Result<Pagination<StartWayOut>, DbErr> {
        let pages = if let Some(user_id) = self.user_id {
            let user_id = entity::prelude::UserId::find_by_id(user_id).one(db).await?;
            let user_id = user_id.unwrap();
            user_id.find_related(entity::prelude::StartWay).order_by_asc(entity::start_way::Column::Id)
                .paginate(db, self.size.get())
        } else {
            entity::start_way::Entity::find().order_by_asc(entity::start_way::Column::Id)
                .paginate(db, self.size.get())
        };
        let total_items = pages.num_items().await?;
        let all_pages = pages.num_pages().await?;
        let items = pages.fetch_page(self.page.get()-1).await?;
        Ok(return_answer::<StartWayOut>(
            items.into_iter().map(|model| model.into()).collect(),
            self, total_items, all_pages
        ))
    }
}

fn return_answer<T: Serialize + Clone + Default>(
    items: Vec<T>,
    data: &Filter,
    total_items: u64,
    all_pages: u64
) -> Pagination<T> {
    Pagination::builder()
        .items(items.into_iter().map(|model| model.into()).collect())
        .total_items(total_items)
        .all_pages(all_pages)
        .size(data.size.get())
        .page(data.page.get())
        .build()
}
