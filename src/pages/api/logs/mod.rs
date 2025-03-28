use actix_web::{Scope, web};
mod loadtest_logs;
mod test;

pub fn logs_scope() -> Scope {
    web::scope("/logs")
        .route("/test", web::get().to(test::test))
        .route("/{id}", web::get().to(loadtest_logs::loadtest_logs))
}
