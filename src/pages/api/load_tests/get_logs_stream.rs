use actix_web::{HttpResponse, Responder, web};
use futures::{StreamExt, stream};
use std::convert::Infallible;
use tokio_stream::wrappers::BroadcastStream;

use crate::{app_state::AppState, utils::monitor_logs::get_or_create_channel};

pub async fn logs_stream(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let load_test_id = path.into_inner();
    let channel = get_or_create_channel(&state, load_test_id)
        .await
        .subscribe();

    let initial_msg = stream::once(async move {
        Ok::<_, Infallible>(web::Bytes::from(
            "event: message\ndata: Initializing log...\n\n",
        ))
    });

    let stream = BroadcastStream::new(channel).map(|msg| {
        Ok::<web::Bytes, Infallible>(match msg {
            Ok(msg) => web::Bytes::from(format!("event: message\ndata: {}\n\n", msg).to_string()),
            Err(_) => web::Bytes::from("data: Unknown error\n\n".to_string()),
        })
    });

    let final_stream = initial_msg.chain(stream);

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/event-stream"))
        .append_header(("Cache-Control", "no-cache"))
        .append_header(("Connection", "keep-alive"))
        .streaming(final_stream)
}
