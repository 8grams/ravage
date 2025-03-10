use actix_web::{Scope, web};

mod get_request_form;
mod post_new_request;

pub fn requests_scope() -> Scope {
    web::scope("/requests")
        .service(web::resource("").route(web::post().to(post_new_request::new_request)))
        .service(web::resource("/{id}").route(web::get().to(get_request_form::request_form)))
}
