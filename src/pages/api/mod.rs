use actix_web::{Scope, web};

mod collections;
mod requests;
mod tabs;

pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(collections::collections_scope())
        .service(requests::requests_scope())
        .service(tabs::tabs_scope())
}
