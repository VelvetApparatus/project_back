use actix_web::{web::{Json, Data}, HttpResponse, HttpRequest};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{models::chat::channel::Channel, utils::cookie_checker::{check, CheckResult}};



#[derive(Deserialize)]
pub struct GetMessagesData {
    pub start_index: i32,
    pub end_index: i32,
}


pub async fn get_messages(
    request: HttpRequest,
    body: Json<Option<GetMessagesData>>,
    pool: Data<PgPool>,
) -> HttpResponse {

    match check(&pool, &request).await {
        CheckResult::BadGateway=> HttpResponse::BadGateway().json("Coludn't get the current user"),
        CheckResult::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        CheckResult::Success(user) => {
            match body.into_inner() {
                None => HttpResponse::BadRequest().json("Body is missing"),
                Some(body) => {
                    match Channel::get_messages(
                        user.user_id.unwrap(), 
                        body.start_index, 
                        body.end_index, 
                        pool
                    ).await {
                        Ok(value) => {
                            HttpResponse::Ok().json(value)
                        },
                        Err(_) => {HttpResponse::BadGateway().finish()}
                    }
                }
            }
        }
    }
}