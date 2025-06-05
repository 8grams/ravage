use actix_web::{Scope, web};

mod callback;
mod google;
mod login;
mod logout;

pub fn auth_scope() -> Scope {
    web::scope("/auth")
        .route("/login", web::post().to(login::login_authentication))
        .route("/google", web::get().to(google::google_auth))
        .route("/callback", web::get().to(callback::google_callback))
        .route("/logout", web::get().to(logout::logout))
}
