use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
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

impl<T: Default + Serialize + Clone> Pagination<T> {
    pub fn builder() -> PaginationBuilder<T> {
        PaginationBuilder::default()
    }
}

#[derive(Default)]
pub struct PaginationBuilder<T> {
    pub items: Vec<T>,
    pub pages: u64,
    pub page: u64,
    pub size: u64,
    pub total: u64,
}

impl<T: Serialize + Clone> PaginationBuilder<T> {
    pub fn items(mut self, items: Vec<T>) -> Self {
        self.items = items;
        self
    }

    pub fn pages(mut self, pages: u64) -> Self {
        self.pages = pages;
        self
    }
    pub fn total(mut self, total: u64) -> Self {
        self.total = total;
        self
    }
    pub fn page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }
    pub fn size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn build(self) -> Pagination<T> {
        Pagination {
            items: self.items,
            pages: self.pages,
            page: self.page,
            size: self.size,
            total: self.total,
        }
    }
}

impl<T: Serialize + Clone> Responder for Pagination<T> {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
