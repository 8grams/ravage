use actix_web::{Scope, web};
mod new_request;
mod request_detail;

pub fn request_scope() -> Scope {
    web::scope("/requests")
        .route("/new", web::get().to(new_request::new_request))
        .route("/{id}", web::get().to(request_detail::request_detail))
}
