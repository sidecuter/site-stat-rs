use crate::errors::{ApiError, ApiResult};
use sea_orm::{DatabaseConnection, EntityTrait, PrimaryKeyTrait};

pub trait FilterTrait<PK> {
    fn filter(
        pk: PK,
        db: &DatabaseConnection,
        msg: String,
    ) -> impl std::future::Future<Output = ApiResult<()>> + Send;
}

impl<T, PK> FilterTrait<PK> for T
where
    T: EntityTrait,
    <T::PrimaryKey as PrimaryKeyTrait>::ValueType: From<PK>,
    PK: Send,
{
    async fn filter(pk: PK, db: &DatabaseConnection, msg: String) -> ApiResult<()> {
        Self::find_by_id(pk)
            .one(db)
            .await?
            .ok_or(ApiError::NotFound(msg))?;
        Ok(())
    }
}
