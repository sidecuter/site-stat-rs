use crate::api::get::route::get_route;
use crate::mut_state::AppStateMutable;
use crate::schemas::filter::FilterRoute;
use crate::schemas::graph::Graph;
use actix_web::web::Data;
use actix_web::{test, App};
use rstest::{fixture, rstest};
use std::sync::Mutex;

static GRAPH_M: &[u8] = include_bytes!("graph_data.bin");

#[fixture]
#[once]
fn shared_state() -> Data<AppStateMutable> {
    let (graph, _): (Graph, usize) =
        bincode::decode_from_slice(&GRAPH_M, bincode::config::standard()).unwrap();
    Data::new(AppStateMutable {
        data_entry: Mutex::new([("M".to_string(), graph)].into_iter().collect()),
    })
}

#[rstest]
#[case::route("m-3301", "m-3501", "campus_M", 200)]
#[case::route("m-3301", "m-3501", "campus_BS", 500)]
#[case::route("m-3301", "m-3101", "campus_M", 404)]
#[actix_web::test]
async fn get_route_endpoint(
    shared_state: &Data<AppStateMutable>,
    #[case] start: &str,
    #[case] end: &str,
    #[case] loc: &str,
    #[case] status: u16,
) {
    let app =
        test::init_service(App::new().app_data(shared_state.clone()).service(get_route)).await;
    let query = FilterRoute {
        from_p: start.to_string(),
        to_p: end.to_string(),
        loc: loc.into(),
    };
    let query = serde_qs::to_string(&query).unwrap();
    let req = test::TestRequest::get()
        .uri(&format!("/route?{query}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), status)
}
