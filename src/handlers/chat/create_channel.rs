use actix_web::{web::{Data, Json}, HttpResponse, HttpRequest};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{utils::cookie_checker::{CheckResult, check}, models::chat::channel::Channel};


#[derive(Debug, Deserialize)]
pub struct CreateChannelData {
    pub name: String,
    pub users: Vec<Uuid>,
}

pub async fn create_channel(
    pool: Data<PgPool>,
    body: Json<Option<CreateChannelData>>,
    request: HttpRequest
) -> HttpResponse {
    match check(&pool, &request).await {
        CheckResult::BadGateway=> HttpResponse::BadGateway().json("Coludn't get the current user"),
        CheckResult::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        CheckResult::Success(user) => {
            match body.into_inner() {
                None => HttpResponse::BadRequest().json("Body is missing"),
                Some(mut body) => {
                    let channel_id = Uuid::new_v4();
                    body.users.push(user.user_id.unwrap());
                    match Channel::create(
                        body.name,
                        body.users,
                        user.user_id.unwrap(),
                        channel_id.clone(),
                        pool
                        ).await {
                        Ok(_) => HttpResponse::Ok().json(channel_id),
                        Err(_) => HttpResponse::Conflict().finish()
                    }
                }
            }

        }
    }

}