use actix::prelude::{Message, Recipient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>, // Web Socket address for this user's connection
    pub room_id: Uuid,              // Room this user is connected to
    pub self_id: Uuid,              // User connection id
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: ActorMessageKind,
    pub room_id: Uuid,
}

impl ClientActorMessage {
    pub fn new(conn_id: Uuid, room_id: Uuid, msg: ActorMessageKind) -> Self {
        Self {
            id: conn_id,
            room_id,
            msg,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ActorMessageKind {
    Me(ActorMessageMe),
    CallUser(ActorMessageCallUser),
    AnswerCall(ActorMessageAnswerCall),
    CallAccepted(ActorMessageCallAccepted),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActorMessageMe {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActorMessageCallUser {
    pub called_by_user_id: Uuid,
    pub called_by_user_name: String,
    pub calling_to_user_id: Uuid,
    pub signal: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActorMessageAnswerCall {
    pub answer_to_user_id: Uuid,
    pub signal: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActorMessageCallAccepted {
    pub signal: String,
}
