use actix_web::{web::{Data, Json}, HttpResponse, HttpRequest};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::chat::{message::Message, channel::Channel}, utils::cookie_checker::{check, CheckResult}};

#[derive(Deserialize)]
pub struct SendMessageBody {
    pub reciever: Uuid,
    pub body: String,
}


pub async fn create_message(
    request: HttpRequest,
    body: Json<Option<SendMessageBody>>,
    pool: Data<PgPool>,
) -> HttpResponse {
    match check(&pool, &request).await {
        CheckResult::BadGateway=> HttpResponse::BadGateway().json("Coludn't get the current user"),
        CheckResult::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        CheckResult::Success(user) => {
            match body.into_inner() {
                None => HttpResponse::BadRequest().json("Body is missing"),
                Some(body) => {
                    let id = Uuid::new_v4();
                    match Message::new(
                        user.user_id.unwrap(),
                        &id,
                        body.reciever, 
                        body.body, 
                        &pool
                    ).await {
                        Ok(_) => {
                             match Channel::update_last_message(&body.reciever, &pool, &id).await {
                                Ok(_) => HttpResponse::Ok().json(id),
                                Err(err) => HttpResponse::InternalServerError().json(err.to_string())
                             }
                        },
                        Err(_) => HttpResponse::Conflict().finish()
                    }
                }
            }
        }
    }


}