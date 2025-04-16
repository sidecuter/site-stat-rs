use crate::errors::ApiResult;
use crate::schemas::{Popular, Status};
use actix_web::{get, web};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityTrait, QueryFilter, QuerySelect, QueryTrait,
};
use sea_orm::sea_query::*;
use crate::entity::{select_aud, start_way};

#[utoipa::path(
    get,
    path = "/api/get/popular",
    responses(
        (
            status = 200, description = "Paginated output for site visits", body = Popular
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        )
    ),
    tag = "Get"
)]
#[get("/popular")]
async fn get_popular(
    db: web::Data<DatabaseConnection>,
) -> ApiResult<Popular> {
    let qr = select_aud::Entity::find()
        .select_only()
        .column_as(select_aud::Column::AuditoryId, "ID")
        .expr_as(select_aud::Column::AuditoryId.count(), "CNT")
        .filter(select_aud::Column::Success.eq(1))
        .group_by(select_aud::Column::AuditoryId)
        .into_query()
        .unions([
            (UnionType::All, start_way::Entity::find()
                .select_only()
                .column_as(start_way::Column::StartId, "ID")
                .expr_as(start_way::Column::StartId.count().mul(3), "CNT")
                .group_by(start_way::Column::StartId)
                .into_query()),
            (UnionType::All, start_way::Entity::find()
                .select_only()
                .column_as(start_way::Column::EndId, "ID")
                .expr_as(start_way::Column::EndId.count().mul(3), "CNT")
                .group_by(start_way::Column::EndId)
                .into_query())
        ]).to_owned();
    let result_query = Query::select()
        .column(Alias::new("ID"))
        .group_by_col(Alias::new("ID"))
        .from_subquery(qr, Alias::new("tr"))
        .order_by_expr(Expr::col(Alias::new("CNT")).sum(), Order::Desc)
        .to_owned();
    let stmt = db.get_database_backend().build(&result_query);
    let results = db.query_all(stmt).await?;
    Ok(Popular(results.into_iter().map(|val| {
        val.try_get_by_index::<String>(0).unwrap()
    }).collect()))
}
