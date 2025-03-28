use actix_web::{Scope, web};

mod collections;
mod load_tests;
mod logs;
mod requests;

pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(collections::collections_scope())
        .service(requests::requests_scope())
        .service(load_tests::load_tests_scope())
        .service(logs::logs_scope())
}
