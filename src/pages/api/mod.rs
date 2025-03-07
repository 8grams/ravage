use actix_web::{Scope, web};

mod collections;

pub fn api_scope() -> Scope {
    web::scope("/api").service(collections::collections_scope())
}
