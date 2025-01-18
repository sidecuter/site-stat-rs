use crate::entity::{change_plan, select_aud, site_stat, start_way, user_id};
use crate::schemas::{Period, Statistics, Target};
use sea_orm::sea_query::SelectStatement;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, QueryTrait, Select,
};

pub enum Query {
    Site(Select<site_stat::Entity>),
    Auds(Select<select_aud::Entity>),
    Ways(Select<start_way::Entity>),
    Plans(Select<change_plan::Entity>),
}

impl Query {
    pub fn generate_query(&self) -> SelectStatement {
        match self {
            Query::Site(q) => q.to_owned().into_query(),
            Query::Auds(q) => q.to_owned().into_query(),
            Query::Ways(q) => q.to_owned().into_query(),
            Query::Plans(q) => q.to_owned().into_query(),
        }
    }

    pub async fn count(
        mut self,
        db: &DatabaseConnection,
        period: &Period,
    ) -> Result<Statistics, DbErr> {
        self = self.filter(period);
        let subquery = self.generate_query();
        let mut statement = user_id::Entity::find()
            .filter(Condition::any().add(user_id::Column::UserId.in_subquery(subquery.clone())))
            .to_owned();
        let visitors = statement.clone().count(db).await?;
        statement = if let Period(Some((ps, pe))) = period {
            statement
                .filter(user_id::Column::CreationDate.between(*ps, *pe))
                .to_owned()
        } else {
            statement
        };
        let unique = statement.count(db).await?;
        let all = self.count_all(db).await?;
        Ok(Statistics {
            unique,
            count: visitors,
            all,
            period: period.to_owned(),
        })
    }

    async fn count_all(self, db: &DatabaseConnection) -> Result<u64, DbErr> {
        match self {
            Query::Site(q) => q.count(db).await,
            Query::Auds(q) => q.count(db).await,
            Query::Ways(q) => q.count(db).await,
            Query::Plans(q) => q.count(db).await,
        }
    }

    pub fn filter(self, period: &Period) -> Self {
        if let Period(Some(period)) = period {
            match self {
                Query::Site(query) => Query::Site(
                    query.filter(site_stat::Column::VisitDate.between(period.0, period.1)),
                ),
                Query::Auds(query) => Query::Auds(
                    query.filter(select_aud::Column::VisitDate.between(period.0, period.1)),
                ),
                Query::Ways(query) => Query::Ways(
                    query.filter(start_way::Column::VisitDate.between(period.0, period.1)),
                ),
                Query::Plans(query) => Query::Plans(
                    query.filter(change_plan::Column::VisitDate.between(period.0, period.1)),
                ),
            }
        } else {
            self
        }
    }
}

impl From<&Target> for Query {
    fn from(value: &Target) -> Self {
        match value {
            Target::Site => Self::Site(
                site_stat::Entity::find()
                    .select_only()
                    .column(site_stat::Column::UserId),
            ),
            Target::Auds => Self::Auds(
                select_aud::Entity::find()
                    .select_only()
                    .column(select_aud::Column::UserId),
            ),
            Target::Ways => Self::Ways(
                start_way::Entity::find()
                    .select_only()
                    .column(start_way::Column::UserId),
            ),
            Target::Plans => Self::Plans(
                change_plan::Entity::find()
                    .select_only()
                    .column(change_plan::Column::UserId),
            ),
        }
    }
}
