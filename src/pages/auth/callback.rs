use crate::{AppState, models::user::User, schema::users};
use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, http::StatusCode, web};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize)]
pub struct QueryParams {
    code: String,
}

pub async fn google_auth_tokeninfo(id_token: &str) -> serde_json::Value {
    let client = reqwest::Client::new();
    let granted_url = format!(
        "https://oauth2.googleapis.com/tokeninfo?id_token={}",
        id_token
    );
    let granted_response = client.get(&granted_url).send().await;
    let granted_json = granted_response.unwrap().json::<serde_json::Value>().await;
    granted_json.unwrap()
}

pub async fn google_callback(
    state: web::Data<AppState>,
    session: Session,
    query_params: web::Query<QueryParams>,
) -> impl Responder {
    let code = query_params.code.as_str();
    let verify_url = "https://oauth2.googleapis.com/token";
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
    let redirect_uri = format!(
        "{}/auth/callback",
        env::var("BASE_URL").expect("BASE_URL must be set")
    );
    let grant_type = "authorization_code".to_string();

    let djson = json!({
        "code": code,
        "client_id": &client_id,
        "client_secret": &client_secret,
        "redirect_uri": &redirect_uri,
        "grant_type": &grant_type,
    });

    let client = reqwest::Client::new();

    let verify_response = client
        .post(verify_url)
        .header("Content-Type", "application/json")
        .json(&djson)
        .send()
        .await;

    let verify_json = verify_response.unwrap().json::<serde_json::Value>().await;
    let vjson = verify_json.unwrap();
    let id_token = vjson.get("id_token").and_then(|t| t.as_str());

    // Check if id_token is present
    if let Some(id_token) = id_token {
        let gjson = google_auth_tokeninfo(id_token).await;

        // Check if user exists
        let pool = &mut state.pool.get().unwrap();
        let result = users::table
            .select(User::as_select())
            .filter(users::email.eq(gjson["email"].as_str().unwrap()))
            .get_result(pool);

        // Create user if not exists
        if let Ok(user_result) = result {
            let session_data = json!({
                "id": user_result.id,
                "name": user_result.name,
                "role": user_result.role,
            });
            let _ = session.insert("session", session_data);

            return HttpResponse::Ok()
                .status(StatusCode::TEMPORARY_REDIRECT)
                .append_header(("Location", "/"))
                .finish();
        } else {
            return HttpResponse::Ok()
                .status(StatusCode::TEMPORARY_REDIRECT)
                .append_header(("Location", "/login?error=2"))
                .finish();
        }
    }

    HttpResponse::Ok()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .append_header(("Location", "/login?error=1"))
        .finish()
}
