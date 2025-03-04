use actix_web::{Scope, web};

mod login;

pub fn auth_scope() -> Scope {
    web::scope("/auth").route("/login", web::post().to(login::login_authentication))
}
