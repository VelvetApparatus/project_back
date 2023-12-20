use actix_web::{HttpRequest, web::{ Data, Query}, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{utils::cookie_checker::{check, CheckResult}, models::chat::user::User};

#[derive(Deserialize)]
pub struct SearchParams {
    pub search_text: String
}

pub async fn search_users(
    request: HttpRequest,
    pool: Data<PgPool>,
    params: Query<SearchParams>
) -> HttpResponse {
    match check(&pool, &request).await {
        CheckResult::BadGateway => HttpResponse::BadGateway().json("Couldn't get the current user"),
        CheckResult::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        CheckResult::Success(_) => {
            match User::search(&params.search_text, &pool).await {
                Err(err) => HttpResponse::Conflict().json(err.to_string()),
                Ok(value) => HttpResponse::Ok().json(value)
            }
        }
    }
}
