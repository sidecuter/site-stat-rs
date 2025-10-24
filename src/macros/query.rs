#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! generate_statistics_query {
    ($table_module:ident, $filter:ident) => {{
        use sea_orm::{JoinType, sea_query::{Expr, Alias, Func, Query}};
        use crate::entity::user_ids::{Entity as UserEntity, Column as UserColumn};
        use $table_module::{Entity as TableEntity, Column as TableColumn};

        let visit_date_expr = Func::cust(DateFunc)
            .arg(Expr::col((TableEntity, TableColumn::VisitDate)));
        let visit_date_out_expr = Func::cast_as(visit_date_expr.clone(), Alias::new("TEXT"));
        let equal_expr = Expr::col((UserEntity, UserColumn::UserId))
            .eq(Expr::col((TableEntity, TableColumn::UserId)));

        let mut mighty_query = Query::select()
            .expr_as(visit_date_out_expr.clone(), Alias::new("period_str"))
            .expr_as(
                Func::count(Expr::col((UserEntity, UserColumn::UserId))),
                Alias::new("all_visits"),
            )
            .expr_as(
                Func::count_distinct(Expr::col((UserEntity, UserColumn::UserId))),
                Alias::new("visitor_count"),
            )
            .expr_as(
                Func::count_distinct(Expr::case(
                    Func::cust(DateFunc)
                        .arg(Expr::col((UserEntity, UserColumn::CreationDate)))
                        .eq(visit_date_expr),
                    Expr::col((UserEntity, UserColumn::UserId)),
                )),
                Alias::new("unique_visits"),
            )
            .join(JoinType::InnerJoin, UserEntity, equal_expr)
            .add_group_by([Expr::expr(visit_date_out_expr.clone()).into_simple_expr()])
            .from(TableEntity)
            .to_owned();

        let mighty_query = match $filter {
            Some(filter) => mighty_query
                .and_where(
                    Expr::col((TableEntity, TableColumn::VisitDate))
                        .gte(filter.start_date),
                )
                .and_where(
                    Expr::col((TableEntity, TableColumn::VisitDate)).lt(filter.end_date),
                )
                .to_owned(),
            None => mighty_query,
        };
        mighty_query
    }};
}
