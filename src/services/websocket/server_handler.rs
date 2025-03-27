use tokio::sync::{mpsc, oneshot};

use super::{ConnId, Msg, server::Command};

#[derive(Debug, Clone)]
pub struct LogServerHandler {
    pub cmd_tx: mpsc::UnboundedSender<Command>,
}

impl LogServerHandler {
    pub async fn connect(&self, conn_tx: mpsc::UnboundedSender<Msg>, log_id: i32) -> ConnId {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::Connect {
                conn_tx,
                res_tx,
                log_id,
            })
            .unwrap();

        res_rx.await.unwrap()
    }

    pub fn disconnect(&self, conn: ConnId) {
        let _ = self.cmd_tx.send(Command::Disconnect { conn });
    }

    pub async fn send_message(&self, log_id: i32, msg: impl Into<Msg>) {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::Message {
                log_id,
                msg: msg.into(),
                res_tx,
            })
            .unwrap();
        let _ = res_rx.await;
    }
}
