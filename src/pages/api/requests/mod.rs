use actix_web::{Scope, web};

mod get_request_form;

pub fn requests_scope() -> Scope {
    web::scope("/requests")
}
