use mockito::{Mock, Server};
use rstest::fixture;

mod fetch_data;
mod parse_data;
mod get_graphs;

#[fixture]
#[once]
fn mock_server() -> (String, Server, Mock) {
    let opts = mockito::ServerOpts {
        port: 8081,
        ..Default::default()
    };
    let mut server = Server::new_with_opts(opts);

    let url = format!("{}/locationsV2.json", server.url());
    let mock = server.mock("GET", "/locationsV2.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(include_str!("locationsV2.json"))
        .expect(3)
        .create();
    (url, server, mock)
}
