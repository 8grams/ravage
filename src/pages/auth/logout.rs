pub async fn logout(session: actix_session::Session) -> impl actix_web::Responder {
    session.remove("session");
    actix_web::HttpResponse::Found()
        .append_header(("Location", "/login"))
        .finish()
}
