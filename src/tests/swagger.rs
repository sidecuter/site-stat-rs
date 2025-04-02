use actix_web::{test, web, App};
use rstest::rstest;
use utoipa_swagger_ui::SwaggerUi;
use crate::api_docs;

#[rstest]
#[tokio::test]
async fn test_swagger() {
    let app = test::init_service(
        App::new()
            .service(
                // OpenAPI document
                web::scope("/docs").service(api_docs::openapi_json).service(
                    SwaggerUi::new("/swagger/{_:.*}").url("/docs/openapi.json", Default::default()),
                ),
            )
    ).await;
    let req = test::TestRequest::get().uri("/docs/openapi.json").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let req = test::TestRequest::get().uri("/docs/swagger/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200)
}
