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
                    println!("sending message to user {id_to}: {m:?}");
                    // let _ = socket_recipient.do_send(WsMessage(m.clone()));
                    match socket_recipient.try_send(WsMessage(m.clone())) {
                        Err(err) => {
                            println!("Error sending client message: {:?}", err);
                        }
                        _ => {}
                    }
                }
                Err(err) => {
                    println!("attempting to send message but couldn't stringify message: {err:?}");
                }
            }
        } else {
            println!(
                "attempting to send message but couldn't find user id {}: {}.",
                id_to,
                self.sessions
                    .keys()
                    .cloned()
                    .map(|k| k.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            );
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
        println!("Disconnecting is {}", msg.id);
        if self.sessions.remove(&msg.id).is_some() {
            // self.rooms
            //     .get(&msg.room_id)
            //     .unwrap()
            //     .iter()
            //     .filter(|conn_id| *conn_id.to_owned() != msg.id)
            //     .for_each(|user_id| {
            //         self.send_message(&format!("{} disconnected.", &msg.id), user_id)
            //     });
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

        // self.rooms
        //     .get(&msg.lobby_id)
        //     .unwrap()
        //     .iter()
        //     .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
        //     .for_each(|conn_id| {
        //         // self.send_message(&format!("{} just joined!", msg.self_id), conn_id)
        //     });

        println!("New connection with id {}", msg.self_id);
        self.sessions.insert(msg.self_id, msg.addr);

        // self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);

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
        println!("ClientActorMessage handler: got message {msg:#?}");
        match msg.msg {
            ActorMessageKind::CallUser(ref inner_msg) => {
                println!("got CallUser message type {inner_msg:#?}");
                self.direct_message(&inner_msg.calling_to_user_id, &msg);
            }
            ActorMessageKind::AnswerCall(ref inner_msg) => {
                println!("got AnswerCall message type {inner_msg:#?}");
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
