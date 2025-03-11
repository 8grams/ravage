use actix_web::{Scope, web};

mod collection_detail;
mod loadtest;

pub fn collections_scope() -> Scope {
    web::scope("/collections")
        .route("/{id}", web::get().to(collection_detail::collection_detail))
        .route("/{id}/loadtest", web::get().to(loadtest::new_load_test))
}
