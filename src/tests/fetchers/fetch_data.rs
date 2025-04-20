use mockito::{Mock, Server};
use rstest::rstest;
use crate::schemas::data::fetch_data;
use super::mock_server;

#[rstest]
#[tokio::test]
async fn test_fetch_data(
    mock_server: &(String, Server, Mock)
) {
    let result = fetch_data(&mock_server.0).await;
    mock_server.2.assert();
    assert!(result.is_ok());
}
