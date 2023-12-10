use actix_web::{HttpRequest, web::{Json, Data}, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{utils::cookie_checker::{check, CheckResult}, models::chat::{user::User, channel::Channel}};



#[derive(Deserialize)]
pub struct OutData {
    pub channel_id: Uuid
}


pub async fn out_channel(
    request: HttpRequest,
    body: Json<Option<OutData>>,
    pool: Data<PgPool>
) -> HttpResponse {
    match check(&pool, &request).await {
        CheckResult::BadGateway=> HttpResponse::BadGateway().json("Coludn't get the current user"),
        CheckResult::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        CheckResult::Success(user) => {
            match body.into_inner() {
                None => HttpResponse::BadRequest().json("Body is missing"),
                Some(body) => {
                    let mut channels: Vec<Uuid> = Vec::new();
                    for chn in user.channels.unwrap() {
                        if chn != body.channel_id {
                            channels.push(chn)
                        }
                    }
                    match User::join_channel(&channels, &user.user_id.unwrap(), &pool).await {
                         Err(err) => HttpResponse::Conflict().json(err.to_string()),
                         Ok(_) => {
                            match Channel::get_by_id(&body.channel_id, &pool).await {
                                Err(err) => HttpResponse::Conflict().json(err.to_string()),
                                Ok(channel) => {
                                    let mut users: Vec<Uuid> = Vec::new();
                                    for usr in channel.users.unwrap() {
                                        if usr != user.user_id.unwrap() {
                                            users.push(usr);
                                        }
                                    }
                                    match Channel::join_user(&users, &body.channel_id, &pool).await {
                                        Err(err) => HttpResponse::Conflict().json(err.to_string()),
                                        Ok(_) => HttpResponse::Ok().finish()
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