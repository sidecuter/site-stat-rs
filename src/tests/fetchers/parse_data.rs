use super::mock_server;
use crate::schemas::data::parse_data;
use mockito::{Mock, Server};
use rstest::rstest;

#[rstest]
#[tokio::test]
async fn test_parse_data(mock_server: &(String, Server, Mock)) {
    let result = parse_data(&mock_server.0).await;
    mock_server.2.assert();
    assert!(result.is_ok());
}
