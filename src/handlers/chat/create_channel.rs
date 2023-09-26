use actix_identity::Identity;
use actix_web::{web::{Data, Json}, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::chat::channel::Channel;


#[derive(Debug, Deserialize)]
pub struct CreateChannelData {
    pub name: String,
    pub users: Vec<Uuid>,
}

pub async fn create_channel(
    pool: Data<PgPool>,
    id: Identity,
    req: Json<Option<CreateChannelData>>
) -> HttpResponse {
    if let Some(id) = id.identity() {
        let mock = req.into_inner();
        match mock {
            Some(mut value) => {
                value.users.push(Uuid::parse_str(&id).unwrap());
                match Channel::create(value.name, value.users, pool).await {
                    Ok(_) => { HttpResponse::Ok().finish() },
                    Err(_) => { HttpResponse::Conflict().finish() }
                }
            },
            None => {HttpResponse::BadRequest().finish()}
        }
    }
    else {
        HttpResponse::Unauthorized().finish()
    }
}