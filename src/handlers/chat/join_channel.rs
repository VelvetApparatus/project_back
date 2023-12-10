use actix_web::{HttpResponse, web::{Data, Json}, HttpRequest};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{utils::cookie_checker::{check, CheckResult}, models::chat::{user::User, channel::Channel}};

#[derive(Deserialize)]
pub struct JoinData { 
    pub channel_id: Uuid
}


pub async fn join_channel(
    request: HttpRequest,
    body: Json<Option<JoinData>>,
    pool: Data<PgPool>
) -> HttpResponse {
    match check(&pool, &request).await {
        CheckResult::BadGateway=> HttpResponse::BadGateway().json("Coludn't get the current user"),
        CheckResult::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        CheckResult::Success(user) => {
            match body.into_inner() {
                None => HttpResponse::BadRequest().json("Body is missing"),
                Some(value) => {
                    let mut channels = user.channels.unwrap();
                    channels.push(value.channel_id);
                     match User::join_channel(&channels, &user.user_id.unwrap(), &pool).await {
                        Err(err) => HttpResponse::Conflict().json(err.to_string()),
                        Ok(_) => {
                            match Channel::get_by_id(&value.channel_id, &pool).await {
                                Err(err) => HttpResponse::Conflict().json(err.to_string()),
                                Ok(channel) => {
                                    let mut  users = channel.users.unwrap();
                                    users.push(user.user_id.unwrap());

                                    match Channel::join_user(&users, &value.channel_id, &pool).await {
                                        Err(err) => HttpResponse::Conflict().json(err.to_string()),
                                        Ok(_) => {HttpResponse::Ok().finish()}
                                    }
                                }
                            }
                        }
                     }
                }
            }
        }
    }
}