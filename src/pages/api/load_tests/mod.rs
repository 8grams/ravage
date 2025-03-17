use actix_web::{Scope, web};
use get_load_test_from::get_load_test_from;

mod get_load_test_from;
mod get_log_stream_ws;
mod get_logs_stream;
mod post_new_loadtest;

pub fn load_tests_scope() -> Scope {
    web::scope("/load_tests")
        .route("", web::post().to(post_new_loadtest::new_loadtest))
        .route("/new", web::get().to(get_load_test_from))
        .route("/{id}/logs", web::get().to(get_logs_stream::logs_stream))
}
