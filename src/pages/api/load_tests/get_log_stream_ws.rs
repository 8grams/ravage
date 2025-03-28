use actix_web::{HttpRequest, HttpResponse, Responder, web};

use crate::{app_state::AppState, services::websocket::handler::log_ws};

pub async fn log_stream_ws(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<i32>,
    state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let log_id = path.into_inner();
    let (res, session, msg_stream) = match actix_ws::handle(&req, stream) {
        Ok(parts) => parts,
        Err(err) => return Ok(HttpResponse::from_error(err)),
    };

    tokio::task::spawn_local(log_ws(
        state.log_server.clone(),
        session,
        msg_stream,
        log_id,
    ));
    Ok(res)
}
