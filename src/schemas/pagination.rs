use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct Pagination<T: Serialize + Clone> {
    pub items: Vec<T>,
    #[schema(example = 10)]
    pub all_pages: u64,
    #[schema(example = 10)]
    pub page: u64,
    #[schema(example = 10)]
    pub size: u64,
    #[schema(example = 91)]
    pub total_items: u64
}

impl<T: Default + Serialize + Clone> Pagination<T> {
    pub fn builder() -> PaginationBuilder<T> {
        PaginationBuilder::default()
    }
}

#[derive(Default)]
pub struct PaginationBuilder<T> {
    pub items: Vec<T>,
    pub all_pages: u64,
    pub page: u64,
    pub size: u64,
    pub total_items: u64
}

impl<T: Serialize + Clone> PaginationBuilder<T> {
    pub fn items(mut self, items: Vec<T>) -> Self {
        self.items = items;
        self
    }

    pub fn all_pages(mut self, all_pages: u64) -> Self {
        self.all_pages = all_pages;
        self
    }
    pub fn total_items(mut self, total_items: u64) -> Self {
        self.total_items = total_items;
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
            all_pages: self.all_pages,
            page: self.page,
            size: self.size,
            total_items: self.total_items,
        }
    }
}

impl<T: Serialize + Clone> Responder for Pagination<T> {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
