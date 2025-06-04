use actix_web::{HttpResponse, Responder, http::StatusCode};
use std::env;

pub async fn google_auth() -> impl Responder {
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
    let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let google_redirect_uri = format!("{}/auth/callback", base_url);
    let scopes = "profile%20email%20openid%20https://www.googleapis.com/auth/userinfo.email";
    let google_auth_uri = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}",
        google_client_id, google_redirect_uri, scopes
    );

    HttpResponse::Ok()
        .status(StatusCode::FOUND)
        .append_header(("Location", google_auth_uri))
        .finish()
}
