use std::collections::{HashMap, HashSet};
use tokio::sync::{mpsc, oneshot};

use super::{ConnId, Msg, RoomId, server_handler::LogServerHandler};

#[derive(Debug)]
pub enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<Msg>,
        res_tx: oneshot::Sender<ConnId>,
        log_id: i32,
    },
    Disconnect {
        conn: ConnId,
    },
    Message {
        log_id: i32,
        msg: Msg,
        res_tx: oneshot::Sender<()>,
    },
}

#[derive(Debug)]
pub struct LogServer {
    session: HashMap<ConnId, mpsc::UnboundedSender<Msg>>,
    rooms: HashMap<RoomId, HashSet<ConnId>>,
}

impl LogServer {
    pub fn new(room: i32) -> (Self, LogServerHandler) {
        let mut rooms = HashMap::with_capacity(4);
        rooms.insert(room, HashSet::new());

        let (cmd_tx, _cmd_rx) = mpsc::unbounded_channel();

        (
            Self {
                session: HashMap::new(),
                rooms,
            },
            LogServerHandler { cmd_tx },
        )
    }

    pub fn send_message(&self, log_id: i32, msg: Msg) {
        if let Some(conns) = self.rooms.get(&log_id) {
            for conn_id in conns {
                if let Some(tx) = self.session.get(conn_id) {
                    let _ = tx.send(msg.clone());
                }
            }
        }
    }
}
