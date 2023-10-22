use std::mem;
use std::time::{Duration, Instant};
use actix::prelude::ContextFutureSpawner;
use actix::{fut, ActorContext, WrapFuture, ActorFutureExt, StreamHandler, Handler};
use actix::{Addr, Actor, AsyncContext};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::models::chat::message::Message;

use super::lobby::Lobby;
use super::messages::{Connect, Disconnect, ClientActorMessage, WsMessage};


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    lobby_addr: Addr<Lobby>,
    heartbeat: Instant,
    session_id: Uuid,
}


impl WsConn {
    pub fn new(lobby: Addr<Lobby>, session_id: Uuid) -> WsConn {
        WsConn { 
            lobby_addr: lobby,
            heartbeat: Instant::now(),
            session_id: session_id
        }
    }

    pub fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                println!("Disconnsecting failed heartbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"hi");
        });
    }

}


impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;


    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(
                Connect {
                    addr: addr.recipient(),
                    self_id: self.session_id,
                }
            )
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }


    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        self.lobby_addr.do_send(Disconnect {
            id: self.session_id,
        });
        actix::Running::Stop
    }
}




impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat= Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(s)) => {
                // println!("UNSAFE START");
                // let ptr: *const u8 = s.as_ptr();
                // let x = ptr.cast::<Message>().as_ref().unwrap();
                // println!("{:?}", s);
                let x = serde_json::from_str::<ClientActorMessage>(&s);
                match x {
                    Ok(value) => {
                        self.lobby_addr.do_send(value)
                    },
                    Err(e) => println!("{:?}", e)
                }
                // println!("UNSAFE STOP");

            },
            Err(e) => std::panic::panic_any(e),
        }
    }
}



impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}