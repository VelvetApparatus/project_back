use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::models::websockets::{lobby::Lobby, connection::WsConn};

#[get("/{session_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    path: Path<(Uuid,)>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let ws = WsConn::new(
        srv.get_ref().clone(),
        path.into_inner().0
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}