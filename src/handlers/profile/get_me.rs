use actix_web::{HttpRequest, web::Data, HttpResponse};
use sqlx::PgPool;

use crate::utils::cookie_checker::{CheckResult, check};




pub async fn get_me(
    request: HttpRequest,
    pool: Data<PgPool>
) -> HttpResponse {
    match check(&pool, &request).await {
        CheckResult::BadGateway=> HttpResponse::BadGateway().json("Coludn't get the current user"),
        CheckResult::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        CheckResult::Success(user) => HttpResponse::Ok().json(user),
    }
}