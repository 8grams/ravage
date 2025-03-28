use actix_ws::{MessageStream, Session};
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
            Some(msg) = conn_rx.recv() => {
                println!("📝 Attempting to send message to client: {}", msg);
                match session.text(msg).await {
                    Ok(_) => println!("✅ Message successfully sent to client."),
                    Err(err) => {
                        eprintln!("❌ Failed to send message to client: {:?}", err);
                        break;
                    }
                }
            }
            Some(msg) = msg_stream.next() => {
                if let Ok(msg) = msg {
                    match msg {
                        actix_ws::Message::Pong(_) => {
                            println!("🔄 Received pong from client, resetting timeout.");
                            last_heartbeat = Instant::now();
                        }
                        actix_ws::Message::Text(text) => {
                            println!("📥 Received message from client: {}", text);
                            // Optionally, you can forward received messages to other clients or process them
                        }
                        _ => println!("📥 Received other message from client"),
                    }
                }
            }
            _ = interval.tick() => {
                if last_heartbeat.elapsed() > CLIENT_TIMEOUT {
                    println!("💀 Client timeout, closing connection");
                    break;
                }
                println!("💓 Sending heartbeat ping");
                let _ = session.ping(b" ").await;
            }
        }
    }

    log_server.disconnect(conn_id, log_id);
    let _ = session.close(None).await;
}
