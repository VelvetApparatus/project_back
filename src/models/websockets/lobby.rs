use crate::models::chat::message::Message;

use super::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::HashMap;
use uuid::Uuid;


type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,       // self id to self
}

impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
        }
    }
}

impl Lobby {
    fn _send_message(&self, message: &Message, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(
                    WsMessage(serde_json::to_string(message).unwrap())
                );
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }


    fn send_connect(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(
                    WsMessage(message.to_string())
                );
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);        
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.sessions.insert(
            msg.self_id,
            msg.addr,
        );
        self.send_connect(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
}
impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        for id in msg.id {
            let message = self.sessions
            .get(&id);
            if message.is_some() {
                message.unwrap().do_send(WsMessage(serde_json::to_string(&msg.msg).unwrap()))
            }
        }
    }
}

