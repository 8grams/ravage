use actix_web::{HttpResponse, Responder};

pub async fn login_authentication() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/"))
        .body("Ok")
}
