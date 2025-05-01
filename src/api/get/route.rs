use crate::errors::{ApiError, ApiResult};
use crate::mut_state::AppStateMutable;
use crate::schemas::filter::FilterRoute;
use crate::schemas::graph::ShortestWay;
use crate::schemas::Status;
use actix_web::{get, web};

#[allow(clippy::significant_drop_tightening)]
#[utoipa::path(
    get,
    path = "/api/get/route",
    params(
        ("from_p" = inline(String), Query, example = "a-102"),
        ("to_p" = inline(String), Query, example = "a-109"),
        ("loc" = inline(String), Query, example = "campus_BS"),
    ),
    responses(
        (
            status = 200, description = "Generated route from one point to another", body = ShortestWay,
            example = json!(
                {
                    "way": [
                        {
                            "id": "a-102",
                            "x": 890,
                            "y": 1581,
                            "type": "entrancesToAu"
                        },
                        {
                            "id": "a-1_21",
                            "x": 890,
                            "y": 1530,
                            "type": "hallway"
                        },
                        {
                            "id": "a-1_22",
                            "x": 884,
                            "y": 1530,
                            "type": "hallway"
                        },
                        {
                            "id": "a-109",
                            "x": 884,
                            "y": 1480,
                            "type": "entrancesToAu"
                        }
                    ],
                    "distance": 107
                }
            )
        ),
        (
            status = 404, description = "Point not found", body = Status,
            example = json!({"status": "You are trying to get a route along non-existent vertex"})
        ),
        (
            status = 500, description = "No such graph", body = Status,
            example = json!(Status{status: "No graphs loaded".to_string()})
        )
    ),
    tag = "Get"
)]
#[get("/route")]
async fn get_route(
    query: web::Query<FilterRoute>,
    app_state: web::Data<AppStateMutable>,
) -> ApiResult<ShortestWay> {
    let graphs_lock = app_state.data_entry.lock()?;
    let graph = graphs_lock
        .get(&query.loc.to_string())
        .ok_or_else(|| ApiError::InternalError(
            "Campus is not available now".to_string(),
        ))?;
    if !graph.has_vertex(&query.from_p) || !graph.has_vertex(&query.to_p) {
        Err(ApiError::NotFound(
            "You are trying to get a route along non-existent vertex".to_string(),
        ))?;
    }
    Ok(graph.get_shortest_way_from_to(&query.from_p, &query.to_p))
}
