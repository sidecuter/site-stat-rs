use sea_orm::{DatabaseConnection, DbErr};
use serde::Serialize;
use crate::schemas::pagination::Pagination;

pub trait Paginate<T: Serialize + Clone> {
    fn pagination(&self, db: &DatabaseConnection) -> impl std::future::Future<Output = Result<Pagination<T>, DbErr>> + Send;
}

macro_rules! impl_paginate_trait {
    ($s_name:ident, $t_name:ident, $entity_path:path, $entity_column:path) => {
        mod paginate {
            use crate::traits::Paginate;
            use crate::schemas::pagination::Pagination;
            use sea_orm::{DbErr, EntityTrait, DatabaseConnection, ModelTrait, QueryOrder, PaginatorTrait};
            use crate::schemas::{$s_name, $t_name};

            impl Paginate<$t_name> for $s_name {
                async fn pagination(&self, db: &DatabaseConnection) -> Result<Pagination<$t_name>, DbErr> {
                    let pages = if let Some(user_id) = self.user_id {
                        let user_id = entity::prelude::UserId::find_by_id(user_id).one(db).await?;
                        let user_id = user_id.unwrap();
                        user_id.find_related($entity_path).order_by_asc($entity_column).paginate(db, self.size.get())
                    } else {
                        <$entity_path as EntityTrait>::find().order_by_asc($entity_column).paginate(db, self.size.get())
                    };
                    let total = pages.num_items().await?;
                    let all_pages = pages.num_pages().await?;
                    let items = pages.fetch_page(self.page.get()-1).await?;

                    Ok(return_answer(
                        items.into_iter().map(|model| model.into()).collect(), self, total, all_pages
                    ))
                }
            }

            fn return_answer(
                items: Vec<$t_name>,
                data: &$s_name,
                total: u64,
                pages: u64
            ) -> Pagination<$t_name> {
                Pagination::builder()
                    .items(items.into_iter().map(|model| model.into()).collect())
                    .total(total)
                    .pages(pages)
                    .size(data.size.get())
                    .page(data.page.get())
                    .build()
            }
        }
    };
}

pub(crate) use impl_paginate_trait;
