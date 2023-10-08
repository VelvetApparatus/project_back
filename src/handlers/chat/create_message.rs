use actix_identity::Identity;
use actix_web::{web::{Data, Json}, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::chat::message::Message;


pub struct SendMessageBody {
    pub reciever: Uuid,
    pub body: String,
}


pub async fn create_message(
    req: Json<Option<SendMessageBody>>,
    id: Identity,
    pool: Data<PgPool>,
) -> HttpResponse {
    let body = req.into_inner().unwrap();
    if let Some(id) = id.identity() {
        match Uuid::parse_str(&id) {
            Ok(id) => {
                match Message::new(id,body.reciever,body.body,pool).await {
                    Ok(_) => {
                        HttpResponse::Ok().finish()
                    },
                    Err(_) => {HttpResponse::BadGateway().finish()}
                    
                }
            },
            Err(_) => { HttpResponse::Unauthorized().finish() }
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }

}