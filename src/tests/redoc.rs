use actix_web::{test, App};
use rstest::rstest;
use utoipa_redoc::{Redoc, Servable};
use utoipa::OpenApi;
use crate::api_docs;

#[rstest]
#[tokio::test]
async fn test_swagger() {
    let app = test::init_service(
        App::new()
            .service(Redoc::with_url("/redoc", api_docs::ApiDoc::openapi())
            )
    ).await;
    let req = test::TestRequest::get().uri("/redoc").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
}
