use actix_identity::Identity;
use actix_web::{web::{Json, Data}, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::chat::channel::Channel;



#[derive(Deserialize)]
pub struct GetMessagesData {
    pub start_index: i32,
    pub end_index: i32,
}


pub async fn get_messages(
    req: Json<Option<GetMessagesData>>,
    pool: Data<PgPool>,
    id: Identity
) -> HttpResponse {
    if let Some(id) = id.identity() {
        let mock = req.into_inner().unwrap();
        let result = Channel::get_messages(
            Uuid::parse_str(&id).unwrap(), 
            mock.start_index, 
            mock.end_index, 
            pool
        ).await;
        match result {
            Ok(value) => {
                HttpResponse::Ok().json(value)
            },
            Err(_) => {HttpResponse::BadGateway().finish()}
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }    

}