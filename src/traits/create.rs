use sea_orm::{DatabaseConnection, DbErr};

pub trait CreateFromScheme<T> {
    fn create(
        &self,
        db: &DatabaseConnection,
    ) -> impl std::future::Future<Output = Result<T, DbErr>> + Send;
}
