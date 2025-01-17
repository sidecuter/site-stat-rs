use sea_orm::{ColumnTrait, DatabaseConnection, LoaderTrait, Select};
use entity::{change_plan, select_aud, start_way, site_stat, prelude::UserId};
use crate::schemas::Period;

pub enum Query {
    Site(Select<site_stat::Entity>),
    Auds(Select<select_aud::Entity>),
    Ways(Select<start_way::Entity>),
    Plans(Select<change_plan::Entity>)
}

impl Query {
    pub fn attach_period(self, period: &Period) -> Self {
        let period = if let Period(Some(period)) = period {
            period
        } else {
            return self
        };
        match self {
            Query::Site(query) => Query::Site(
                query
                    .filter(site_stat::Column::VisitDate.gte(period.0))
                    .filter(site_stat::Column::VisitDate.lte(period.1))
            ),
            Query::Auds(query) => Query::Auds(
                query
                    .filter(select_aud::Column::VisitDate.gte(period.0))
                    .filter(select_aud::Column::VisitDate.lte(period.1))
            ),
            Query::Ways(query) => Query::Ways(
                query
                    .filter(start_way::Column::VisitDate.gte(period.0))
                    .filter(start_way::Column::VisitDate.lte(period.1))
            ),
            Query::Plans(query) => Query::Plans(
                query
                    .filter(change_plan::Column::VisitDate.gte(period.0))
                    .filter(change_plan::Column::VisitDate.lte(period.1))
            ),
        }
    }

    pub async fn get_users(self, db: &DatabaseConnection) -> crate::errors::Result<Vec<entity::user_id::Model>> {
        let users = match self {
            Query::Site(query) => query.all(db).await?.load_one(UserId, db).await?,
            Query::Auds(query) => query.all(db).await?.load_one(UserId, db).await?,
            Query::Ways(query) => query.all(db).await?.load_one(UserId, db).await?,
            Query::Plans(query) => query.all(db).await?.load_one(UserId, db).await?,
        };
        Ok(users.into_iter().map(|v| v.unwrap()).collect())
    }
}
