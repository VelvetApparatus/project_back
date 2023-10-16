use actix_web::{web::{Data, Json}, HttpResponse, HttpRequest};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::chat::message::Message, utils::cookie_checker::{check, CheckResult}};


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
                    match Message::new(user.user_id.unwrap(), body.reciever, body.body, pool).await {
                        Ok(_) => HttpResponse::Ok().finish(),
                        Err(_) => HttpResponse::Conflict().finish()
                    }
                }
            }
        }
    }


}