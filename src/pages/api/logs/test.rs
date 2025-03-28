use actix_web::{HttpRequest, HttpResponse, Responder, web};
use tokio::time::{Duration, sleep};

use crate::{app_state::AppState, services::websocket::handler::log_ws};

pub async fn test(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let log_id = 0;
    let (res, session, msg_stream) = match actix_ws::handle(&req, stream) {
        Ok(parts) => parts,
        Err(err) => return Ok(HttpResponse::from_error(err)),
    };

    let handler_clone = state.log_server.clone();

    let mut count = 0;
    tokio::spawn(async move {
        loop {
            count += 1;
            sleep(Duration::from_nanos(1)).await;
            handler_clone
                .send_message(log_id, format!("<div id='logs' hx-swap-oob='beforeend'><pre><code>🔥 Server Log {}: Update every 1 nano seconds!</code></pre></div>",count).to_string())
                .await;
            if count >= 1000_000_000 {
                break;
            }
        }
    });
    tokio::task::spawn_local(log_ws(
        state.log_server.clone(),
        session,
        msg_stream,
        log_id,
    ));
    Ok(res)
}
