use actix_web::{Scope, web};
use get_load_test_from::get_load_test_from;

mod get_load_test_from;
mod post_new_loadtest;

pub fn load_tests_scope() -> Scope {
    web::scope("/load_tests")
        .route("", web::post().to(post_new_loadtest::new_loadtest))
        .route("/new", web::get().to(get_load_test_from))
}
