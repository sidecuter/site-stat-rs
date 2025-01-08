use serde::Deserialize;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ModelTrait, PaginatorTrait, QueryOrder};
use utoipa::ToSchema;
use crate::schemas::validators::{ApiKey, Page, Size};
use crate::schemas::pagination::{Pagination, PaginationBuilder};
use crate::schemas::site_stat::SiteStatisticsOut;
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
        let builder: PaginationBuilder<SiteStatisticsOut> = Pagination::builder();
        Ok(builder
            .items(items.into_iter().map(|model| model.into()).collect())
            .total_items(total_items)
            .all_pages(all_pages)
            .size(self.size.get())
            .page(self.page.get())
            .build()
        )
    }
}
