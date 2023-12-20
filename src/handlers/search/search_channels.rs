use actix_web::{HttpRequest, web::{Data, Query}, HttpResponse};
use sqlx::PgPool;

use crate::{models::chat::channel::Channel, utils::cookie_checker::{check, CheckResult}};

use super::search_users::SearchParams;

pub async fn search_channels(
    request: HttpRequest,
    pool: Data<PgPool>,
    params: Query<SearchParams>
) -> HttpResponse {
    match check(&pool, &request).await {
        CheckResult::BadGateway => HttpResponse::BadGateway().json("Couldn't get the current user"),
        CheckResult::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        CheckResult::Success(_) => {
            match Channel::search(&params.search_text, &pool).await {
                Err(err) => HttpResponse::Conflict().json(err.to_string()),
                Ok(value) => HttpResponse::Ok().json(value)
            }
        }
    }
}
