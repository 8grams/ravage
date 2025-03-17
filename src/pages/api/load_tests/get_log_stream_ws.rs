use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_ws::Message;

pub async fn log_stream_ws(
    req: HttpRequest,
    body: web::Payload,
) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;
    Ok(response)
}
