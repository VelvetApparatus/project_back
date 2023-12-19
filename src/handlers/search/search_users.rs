use actix_web::{HttpRequest, web::{Json, Data}, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{utils::cookie_checker::{check, CheckResult}, models::chat::user::User};



#[derive(Deserialize)]
pub struct SearchBody {
    pub search_text: String
}

pub async fn search_users(
    request: HttpRequest,
    pool: Data<PgPool>,
    body: Json<Option<SearchBody>>
) -> HttpResponse {
    match check(&pool, &request).await {
        CheckResult::BadGateway=> HttpResponse::BadGateway().json("Coludn't get the current user"),
        CheckResult::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        CheckResult::Success(user) => {
            match body.into_inner() {
                None => HttpResponse::Conflict().json("Body is missing"),
                Some(body) => {
                     match User::search(&body.search_text, &pool).await {
                        Err(err) => HttpResponse::Conflict().json(err.to_string()),
                        Ok(value) => HttpResponse::Ok().json(value)
                     }
                }
            }
        }
    }
}