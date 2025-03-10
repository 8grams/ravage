use actix_web::{Scope, web};
mod new_request;

pub fn request_scope() -> Scope {
    web::scope("/requests").route("/new", web::get().to(new_request::new_request))
}
