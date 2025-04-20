use mockito::{Mock, Server};
use rstest::rstest;
use crate::schemas::data::get_graphs;
use super::mock_server;

#[rstest]
#[tokio::test]
async fn test_parse_data(
    mock_server: &(String, Server, Mock)
) {
    let result = get_graphs().await;
    mock_server.2.assert();
    assert!(result.is_ok());
}
