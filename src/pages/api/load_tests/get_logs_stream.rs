use actix_web::{HttpResponse, Responder, web};
use futures::{StreamExt, stream};
use std::{collections::VecDeque, convert::Infallible, sync::Mutex};
use tokio_stream::wrappers::BroadcastStream;

use crate::{app_state::AppState, utils::monitor_logs::get_or_create_channel};

pub async fn logs_stream(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let load_test_id = path.into_inner();
    let channel = get_or_create_channel(&state, load_test_id)
        .await
        .subscribe();

    // Mutex-wrapped VecDeque to store the last 500 log lines
    let logs_buffer = web::Data::new(Mutex::new(VecDeque::with_capacity(500)));

    let initial_msg = stream::once(async move {
        Ok::<_, Infallible>(web::Bytes::from("data: Initializing log...\n\n"))
    });

    let stream = BroadcastStream::new(channel).map(move |msg| {
        let mut buffer = logs_buffer.lock().unwrap();

        match msg {
            Ok(msg) => {
                // Push new message
                buffer.push_back(msg.clone());

                // Remove old messages if over 500
                if buffer.len() > 500 {
                    buffer.pop_front();
                }

                Ok::<web::Bytes, Infallible>(web::Bytes::from(format!("{}\n\n", msg)))
            }
            Err(_) => Ok(web::Bytes::from("data: Unknown error\n\n")),
        }
    });

    let final_stream = initial_msg.chain(stream);

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/event-stream"))
        .append_header(("Cache-Control", "no-cache"))
        .append_header(("Connection", "keep-alive"))
        .streaming(final_stream)
}
