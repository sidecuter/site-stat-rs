use sea_orm::{DatabaseConnection, DbErr};

pub trait Paginate<T> {
    fn pagination(&self, db: &DatabaseConnection) -> impl std::future::Future<Output = Result<T, DbErr>> + Send;
}
