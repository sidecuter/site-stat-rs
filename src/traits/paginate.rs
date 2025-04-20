use crate::schemas::{Filter, Pagination};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, FromQueryResult, PaginatorTrait, Select};
use serde::Serialize;

pub trait Paginate<'db, E, M>
where
    E: EntityTrait<Model = M>,
    M: FromQueryResult + Sized + Send + Sync + 'db,
    Self: Serialize + Clone + From<M>,
{
    fn pagination(
        db: &DatabaseConnection,
        filter: &Filter,
    ) -> impl std::future::Future<Output = Result<Pagination<Self>, DbErr>> + Send {
        async {
            let query = Self::get_query(filter);
            let pages = query.paginate(db, filter.size);
            let total = pages.num_items().await?;
            let all_pages = pages.num_pages().await?;
            let items = pages.fetch_page(filter.page - 1).await?;

            Ok(Self::return_answer(
                items.into_iter().map(|model| model.into()).collect(),
                filter,
                total,
                all_pages,
            ))
        }
    }

    fn get_query(filter: &Filter) -> Select<E>;
    fn return_answer(items: Vec<Self>, data: &Filter, total: u64, pages: u64) -> Pagination<Self> {
        Pagination::new(items, data.page, data.size, total, pages)
    }
}
