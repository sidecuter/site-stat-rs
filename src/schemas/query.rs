use crate::entity::{change_plan, select_aud, site_stat, start_way, user_id};
use crate::schemas::{Period, Statistics, Target};
use crate::{build_query, filter_visit};
use sea_orm::{
    sea_query::SelectStatement, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    PaginatorTrait, QueryFilter, QuerySelect, QueryTrait, Select,
};

pub enum Query {
    Site(Select<site_stat::Entity>),
    Auds(Select<select_aud::Entity>),
    Ways(Select<start_way::Entity>),
    Plans(Select<change_plan::Entity>),
}

impl Query {
    #[must_use]
    pub fn generate_query(&self) -> SelectStatement {
        match self {
            Self::Site(q) => q.to_owned().into_query(),
            Self::Auds(q) => q.to_owned().into_query(),
            Self::Ways(q) => q.to_owned().into_query(),
            Self::Plans(q) => q.to_owned().into_query(),
        }
    }

    /// Counts statistics for specified endpoint
    ///
    /// # Errors
    /// db errors
    pub async fn count(
        mut self,
        db: &DatabaseConnection,
        period: &Period,
    ) -> Result<Statistics, DbErr> {
        self = self.apply_period_filter(period);

        let subquery = self.generate_query();
        let base_query =
            user_id::Entity::find().filter(user_id::Column::UserId.in_subquery(subquery));

        let visitors = base_query.clone().count(db).await?;

        let unique_query = match period {
            Period(Some((start, end))) => base_query
                .clone()
                .filter(user_id::Column::CreationDate.between(*start, *end)),
            _ => base_query.clone(),
        };

        let unique = unique_query.count(db).await?;
        let all = self.count_query(db).await?;

        Ok(Statistics {
            unique,
            count: visitors,
            all,
            period: period.clone(),
        })
    }

    async fn count_query(self, db: &DatabaseConnection) -> Result<u64, DbErr> {
        match self {
            Self::Site(q) => q.count(db).await,
            Self::Auds(q) => q.count(db).await,
            Self::Ways(q) => q.count(db).await,
            Self::Plans(q) => q.count(db).await,
        }
    }

    #[must_use]
    pub fn apply_period_filter(self, period: &Period) -> Self {
        let Some((start, end)) = period.0 else {
            return self;
        };

        filter_visit!(self, start, end; {
            Site => site_stat,
            Auds => select_aud,
            Ways => start_way,
            Plans => change_plan,
        })
    }
}

impl From<&Target> for Query {
    fn from(value: &Target) -> Self {
        build_query!(value => Self {
                Site => site_stat,
                Auds => select_aud,
                Ways => start_way,
                Plans => change_plan,
            }
        )
    }
}
