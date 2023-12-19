use actix_web::{HttpRequest, web::{Data, Json}, HttpResponse};
use sqlx::PgPool;

use crate::{models::chat::channel::Channel, utils::cookie_checker::{check, CheckResult}};

use super::search_users::SearchBody;



pub async fn search_channels(
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
                     match Channel::search(&body.search_text, &pool).await {
                        Err(err) => HttpResponse::Conflict().json(err.to_string()),
                        Ok(value) => HttpResponse::Ok().json(value)
                     }
                }
            }
        }
    }
}