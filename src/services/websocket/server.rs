use std::collections::{HashMap, HashSet};

use tokio::sync::{mpsc, oneshot};

use super::{ConnId, Msg, RoomId};

#[derive(Debug)]
pub struct LogServer {
    session: HashMap<ConnId, mpsc::UnboundedSender<Msg>>,
    rooms: HashMap<RoomId, HashSet<ConnId>>,
}

impl LogServer {
    pub fn new(room: i32) -> (Self, LogServerHandle) {
        let mut rooms = HashMap::with_capacity(4);

        rooms.insert(room.to_owned(), HashSet::new());

        (
            Self {
                session: HashMap::new(),
                rooms,
            },
            LogServerHandle,
        )
    }
    pub fn send_system_message(&self, room: i32, skip: ConnId, msg: impl Into<Msg>) {
        if let Some(session) = self.rooms.get(&room) {
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
}

#[derive(Debug, Clone)]
pub struct LogServerHandle;
