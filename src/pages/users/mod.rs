use actix_web::{Scope, web};
mod index;
pub fn users_page_scope() -> Scope {
    return web::scope("/users").route("", web::get().to(index::users_page));
}
