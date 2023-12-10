
use actix_web::{HttpRequest, web::{Data, Json}, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{utils::cookie_checker::{check, CheckResult}, models::chat::user::User};


#[derive(Deserialize)]
pub struct UpdateMeData {
    background: String,
    icon: String,
    username: String,
}


pub async fn update_me(
    request: HttpRequest,
    pool: Data<PgPool>,
    body: Json<Option<UpdateMeData>>
) -> HttpResponse {
    match check(&pool, &request).await {
        CheckResult::BadGateway=> HttpResponse::BadGateway().json("Coludn't get the current user"),
        CheckResult::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        CheckResult::Success(user) => {
            match body.into_inner() {
                None => HttpResponse::Conflict().json("Body is missing"),
                Some(body) => {
                    match User::update_user(&user.user_id.unwrap(), &body.icon, &body.background, &body.username, &pool).await {
                        Err(err) => HttpResponse::Conflict().json(err.to_string()),
                        Ok(_) => HttpResponse::Ok().finish()

                    }

                }
            }
        }
    }
}