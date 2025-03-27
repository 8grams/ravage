use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
use std::time::{Duration, Instant};
use tokio::{sync::mpsc, time::interval};

use super::server_handler::LogServerHandler;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn log_ws(
    log_server: LogServerHandler,
    mut session: Session,
    mut msg_stream: MessageStream,
    log_id: i32,
) {
    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();
    let conn_id = log_server.connect(conn_tx, log_id).await;

    loop {
        tokio::select! {
            // Handle messages from the server and send them to the client
            Some(msg) = conn_rx.recv() => {
                if session.text(msg).await.is_err() {
                    break; // Stop on error
                }
            }

            // Ignore messages from the client
            Some(_) = msg_stream.next() => {
                // Clients cannot send messages, so we ignore them
            }

            // Send heartbeat and check for timeouts
            _ = interval.tick() => {
                if last_heartbeat.elapsed() > CLIENT_TIMEOUT {
                    break; // Close connection if no heartbeat response
                }
                let _ = session.ping(b"").await;
            }
        }
    }

    log_server.disconnect(conn_id);
    let _ = session.close(None).await;
}
