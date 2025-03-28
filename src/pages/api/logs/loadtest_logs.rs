use actix_web::{HttpRequest, HttpResponse, Responder, web};

use crate::{app_state::AppState, services::websocket::handler::log_ws};

pub async fn loadtest_logs(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let log_id = path.into_inner();
    let (res, session, msg_stream) = match actix_ws::handle(&req, stream) {
        Ok(parts) => parts,
        Err(err) => return Ok(HttpResponse::from_error(err)),
    };

    let sender = state.log_server.clone();
    let _ = sender
        .send_message(
            log_id,
            "<div id='logs'><pre><code>Initializing logs...</code></pre></div>".to_string(),
        )
        .await;

    tokio::task::spawn_local(log_ws(
        state.log_server.clone(),
        session,
        msg_stream,
        log_id,
    ));
    Ok(res)
}
