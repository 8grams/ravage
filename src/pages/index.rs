use actix_web::{HttpResponse, Responder, web};

use crate::app_state::AppState;
pub async fn main_pages(data: web::Data<AppState>) -> impl Responder {
    let conn = data.pool.get();

    HttpResponse::Ok().body("Hello")
}
