use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone, ToSchema)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Pagination<T: Serialize + Clone> {
    pub items: Vec<T>,
    #[schema(example = 10)]
    pub pages: u64,
    #[schema(example = 10)]
    pub page: u64,
    #[schema(example = 10)]
    pub size: u64,
    #[schema(example = 91)]
    pub total: u64,
}

impl<T: Serialize + Clone> Pagination<T> {
    pub fn new(items: Vec<T>, page: u64, size: u64, total: u64, pages: u64) -> Self {
        Self {
            items,
            pages,
            page,
            size,
            total,
        }
    }
}

impl<T: Serialize + Clone> Responder for Pagination<T> {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
