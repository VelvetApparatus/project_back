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
// =================================================================================
// REFACTOR CHANNELS STRUCT (ADD CREATOR_ID )
// =================================================================================

pub async fn create_channel(
    pool: Data<PgPool>,
    id: Identity,
    req: Json<Option<CreateChannelData>>
) -> HttpResponse {
    if let Some(id) = id.identity() {
        match Uuid::parse_str(&id) {
            Ok(uid) => {
                let mock = req.into_inner();
                match mock {
                    Some(mut value) => {
                        // Adding Uuid of creator
                        value.users.push(uid.clone());

                        match Channel::create(value.name, value.users, uid, pool).await {
                            Ok(_) => { HttpResponse::Ok().finish() },
                            Err(_) => { HttpResponse::Conflict().finish() }
                        }
                    },
                    None => {HttpResponse::BadRequest().finish()}
                }
            },
            Err(_) => {HttpResponse::Unauthorized().finish()}
        }
    }
    else {
        HttpResponse::Unauthorized().finish()
    }
}