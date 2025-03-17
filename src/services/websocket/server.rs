use futures::io;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use tokio::sync::{mpsc, oneshot};

use super::{ConnId, Msg, RoomId};

#[derive(Debug)]
enum Command {
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
    cmd_rx: mpsc::UnboundedReceiver<Command>,
}

impl LogServer {
    pub fn new(room: i32) -> (Self, LogServerHandler) {
        let mut rooms = HashMap::with_capacity(4);

        rooms.insert(room.to_owned(), HashSet::new());
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        (
            Self {
                session: HashMap::new(),
                rooms,
                cmd_rx,
            },
            LogServerHandler { cmd_tx },
        )
    }
    pub fn send_system_message(&self, log_id: i32, skip: ConnId, msg: impl Into<Msg>) {
        if let Some(session) = self.rooms.get(&log_id) {
            let msg = msg.into();

            for conn_id in session {
                if *conn_id != skip {
                    if let Some(tx) = self.session.get(conn_id) {
                        let _ = tx.send(msg.clone());
                    }
                }
            }
        }
    }
    pub fn connect(&mut self, tx: mpsc::UnboundedSender<Msg>, log_id: i32) -> ConnId {
        let id = rand::rng().random::<ConnId>();
        self.session.insert(id, tx);

        self.rooms.entry(log_id.to_owned()).or_default().insert(id);

        id
    }

    pub fn disconnect(&mut self, conn_id: ConnId) {
        self.session.remove(&conn_id);
    }

    pub async fn run(mut self) -> io::Result<()> {
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Command::Connect {
                    conn_tx,
                    res_tx,
                    log_id,
                } => {
                    let conn_id = self.connect(conn_tx, log_id);
                }
                Command::Disconnect { conn } => {
                    self.disconnect(conn);
                }
                Command::Message {
                    log_id,
                    msg,
                    res_tx,
                } => {
                    self.send_system_message(log_id, 0, msg);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LogServerHandler {
    cmd_tx: mpsc::UnboundedSender<Command>,
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
        self.cmd_tx.send(Command::Disconnect { conn }).unwrap();
    }
    pub async fn send_msesage(&self, log_id: i32, msg: impl Into<Msg>) {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::Message {
                log_id,
                msg: msg.into(),
                res_tx,
            })
            .unwrap();
        res_rx.await.unwrap();
    }
}
