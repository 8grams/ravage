use actix_web::{web, HttpResponse, Responder};
use futures::{stream, StreamExt};
use std::convert::Infallible;
use tokio_stream::wrappers::BroadcastStream;

use crate::{app_state::AppState, utils::monitor_logs::get_or_create_channel};

pub async fn logs_stream(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let load_test_id = path.into_inner();
    let channel = get_or_create_channel(&state, load_test_id)
        .await
        .subscribe();

    let initial_msg = stream::once(async {
        Ok::<_, Infallible>(web::Bytes::from(
            "data: <pre><code>Initializing log...</code></pre>\n\n",
        ))
    });

    let stream = BroadcastStream::new(channel).filter_map(|msg| async move {
        match msg {
            Ok(msg) => Some(Ok(web::Bytes::from(msg))),
            Err(_) => Some(Ok(web::Bytes::from(
                "data: <pre><code>Unknown error</code></pre>\n\n",
            ))),
        }
    });

    let final_stream = initial_msg.chain(stream);

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/event-stream"))
        .append_header(("Cache-Control", "no-cache"))
        .append_header(("Connection", "keep-alive"))
        .streaming(final_stream)
}
