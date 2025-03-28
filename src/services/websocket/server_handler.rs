use super::{ConnId, Msg, RoomId, server::Command};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Clone)]
pub struct LogServerHandler {
    pub cmd_tx: mpsc::UnboundedSender<Command>,
}

impl LogServerHandler {
    pub async fn connect(&self, conn_tx: mpsc::UnboundedSender<Msg>, log_id: RoomId) -> ConnId {
        let (res_tx, res_rx) = oneshot::channel();
        if self
            .cmd_tx
            .send(Command::Connect {
                conn_tx,
                res_tx,
                log_id,
            })
            .is_err()
        {
            eprintln!("Failed to send connect command");
            return 0;
        }
        res_rx.await.unwrap_or(0)
    }

    pub fn disconnect(&self, conn: ConnId, log_id: RoomId) {
        let _ = self.cmd_tx.send(Command::Disconnect { conn, log_id });
    }

    pub async fn send_message(&self, log_id: RoomId, msg: impl Into<Msg>) {
        let (res_tx, res_rx) = oneshot::channel();
        if self
            .cmd_tx
            .send(Command::Message {
                log_id,
                msg: msg.into(),
                res_tx,
            })
            .is_err()
        {
            eprintln!("Failed to send message");
            return;
        }
        let _ = res_rx.await;
    }
}
