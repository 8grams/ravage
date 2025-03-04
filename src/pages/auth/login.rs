use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

pub async fn login_authentication(
    form_data: web::Form<LoginForm>,
    session: actix_session::Session,
) -> impl Responder {
    let username = env::var("USERNAME").unwrap_or("admin".to_string());
    let password = env::var("PASSWORD").unwrap_or("admin".to_string());
    let form_data = form_data.into_inner();
    if (form_data.username == username) && (form_data.password == password) {
        let _ = session.insert("session", json!({ "username": username }));
        return HttpResponse::Found()
            .append_header(("Location", "/"))
            .append_header(("Hx-Location", "/"))
            .body("Ok");
    }

    HttpResponse::Unauthorized().body("Username or password is incorrect!")
}
