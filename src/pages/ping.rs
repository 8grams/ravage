use actix_web::{HttpResponse, Responder};

pub async fn main() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
