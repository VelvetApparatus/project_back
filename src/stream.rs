
use actix::{Actor, StreamHandler};
use actix_web::{HttpRequest, web, HttpResponse, Error};
use actix_web_actors::ws;


struct MyWs{
    // user_id: Uuid,
    // channel_id: Uuid, // DELETE ???
}


impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("TEXT");
                println!("{:#?}", text);
                ctx.text(text)
            },
            Ok(ws::Message::Binary(bin)) => {
                println!("BINARY");
                println!("{:?}", bin);
                ctx.binary(bin)
            },
            _ => (),
        }
    }
}

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}