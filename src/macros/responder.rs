#[macro_export]
macro_rules! impl_responder {
    ($ret_type:ty) => {
        impl Responder for $ret_type {
            type Body = BoxBody;

            fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
                actix_web::HttpResponse::Ok().json(self)
            }
        }
    };
}
