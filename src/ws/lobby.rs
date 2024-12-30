use crate::ws::messages::{
    ActorMessageCallAccepted, ActorMessageKind, ActorMessageMe, ClientActorMessage, Connect,
    Disconnect, WsMessage,
};
use actix::prelude::{Actor, Context, Handler, Recipient};
use serde_json::to_string;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

type Socket = Recipient<WsMessage>;

// Lobby (aka Connections Hub)
pub struct Lobby {
    sessions: HashMap<Uuid, Socket>, // user connection id -> connection
    rooms: HashMap<Uuid, HashSet<Uuid>>, // room id -> set of user connection ids
}

impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl Lobby {
    fn send_message(&self, id_to: &Uuid, message: &ClientActorMessage) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            match to_string(&message.msg) {
                Ok(m) => {
                    // let _ = socket_recipient.do_send(WsMessage(m.clone()));
                    match socket_recipient.try_send(WsMessage(m.clone())) {
                        Err(err) => {
                            log::error!("Error sending client message: {:?}", err);
                        }
                        _ => {}
                    }
                }
                Err(err) => {
                    log::error!(
                        "attempting to send message but couldn't stringify message: {err:?}"
                    );
                }
            }
        }
    }

    fn direct_message(&mut self, dest_conn_id: &Uuid, msg: &ClientActorMessage) {
        self.send_message(dest_conn_id, msg);
    }

    fn broadcast_message(&mut self, msg: &ClientActorMessage) {
        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|client| self.send_message(client, &msg));
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.rooms
            .entry(msg.room_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);

        self.sessions.insert(msg.self_id, msg.addr);

        self.direct_message(
            &msg.self_id,
            &ClientActorMessage {
                id: msg.self_id,
                room_id: msg.room_id,
                msg: ActorMessageKind::Me(ActorMessageMe { id: msg.self_id }),
            },
        );
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        match msg.msg {
            ActorMessageKind::CallUser(ref inner_msg) => {
                self.direct_message(&inner_msg.calling_to_user_id, &msg);
            }
            ActorMessageKind::AnswerCall(ref inner_msg) => {
                self.direct_message(
                    &inner_msg.answer_to_user_id,
                    &ClientActorMessage {
                        id: msg.id,
                        room_id: msg.room_id,
                        msg: ActorMessageKind::CallAccepted(ActorMessageCallAccepted {
                            signal: inner_msg.signal.clone(),
                        }),
                    },
                );
            }
            ActorMessageKind::Me(_) | ActorMessageKind::CallAccepted(_) => {
                unreachable!()
            }
        }
    }
}
