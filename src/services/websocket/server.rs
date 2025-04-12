
use super::{ConnId, Msg, RoomId, server_handler::LogServerHandler};
use std::collections::{HashMap, HashSet};
use tokio::{
    sync::{mpsc, oneshot},
    task,
};

#[derive(Debug)]
pub enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<Msg>,
        res_tx: oneshot::Sender<ConnId>,
        log_id: RoomId,
    },
    Disconnect {
        conn: ConnId,
        log_id: RoomId,
    },
    Message {
        log_id: RoomId,
        msg: Msg,
        res_tx: oneshot::Sender<()>,
    },
}

#[derive(Debug, Clone)]
pub struct LogServer {
    session: HashMap<ConnId, mpsc::UnboundedSender<Msg>>,
    rooms: HashMap<RoomId, HashSet<ConnId>>,
    next_conn_id: ConnId,
}

impl LogServer {
    pub fn new() -> (Self, LogServerHandler) {
        let mut rooms = HashMap::with_capacity(4);
        rooms.insert(0, HashSet::new());
        let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel();

        let server = Self {
            session: HashMap::new(),
            rooms,
            next_conn_id: 1,
        };

        let mut server_clone = server.clone();
        task::spawn(async move {
            while let Some(command) = cmd_rx.recv().await {
                match command {
                    Command::Connect {
                        conn_tx,
                        res_tx,
                        log_id,
                    } => {
                        let conn_id = server_clone.handle_connect(conn_tx, log_id);
                        let _ = res_tx.send(conn_id);
                        println!("🔌 New Connection: log_id={}, conn_id={}", log_id, conn_id);
                    }
                    Command::Disconnect { conn, log_id } => {
                        server_clone.handle_disconnect(conn, log_id);
                        println!("❌ Disconnected: log_id={}, conn_id={}", log_id, conn);
                    }
                    Command::Message {
                        log_id,
                        msg,
                        res_tx,
                    } => {
                        server_clone.handle_message(log_id, msg);
                        let _ = res_tx.send(());
                        println!("📩 Message sent to log_id={}", log_id);
                    }
                }
            }
        });

        (server, LogServerHandler { cmd_tx })
    }

    fn handle_connect(&mut self, conn_tx: mpsc::UnboundedSender<Msg>, log_id: RoomId) -> ConnId {
        let conn_id = self.next_conn_id;
        self.next_conn_id += 1;

        // Add connection to session
        self.session.insert(conn_id, conn_tx);

        // Add connection to room
        self.rooms
            .entry(log_id)
            .or_default()
            .insert(conn_id);

        conn_id
    }

    fn handle_disconnect(&mut self, conn: ConnId, log_id: RoomId) {
        // Remove from session
        self.session.remove(&conn);

        // Remove from room
        if let Some(room) = self.rooms.get_mut(&log_id) {
            room.remove(&conn);
        }
    }

    fn handle_message(&mut self, log_id: RoomId, msg: Msg) {
        // Get connections for this log_id
        if let Some(room_connections) = self.rooms.get(&log_id) {
            for &conn_id in room_connections {
                // Try to send message to each connection in the room
                if let Some(tx) = self.session.get(&conn_id) {
                    let _ = tx.send(msg.clone());
                }
            }
        }
    }

    pub fn send_message(&mut self, log_id: RoomId, msg: Msg) {
        if let Some(conns) = self.rooms.get(&log_id) {
            for conn_id in conns {
                if let Some(tx) = self.session.get(conn_id) {
                    let _ = tx.send(msg.clone());
                }
            }
        }
    }
}
