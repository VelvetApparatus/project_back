use actix_web::{web::Data, HttpResponse, HttpRequest};
use sqlx::PgPool;
use crate::{utils::cookie_checker::{CheckResult, check}, models::chat::user::User};



pub async fn show_channels(
    request: HttpRequest,
    pool: Data<PgPool>,
) -> HttpResponse {
    match check(&pool, &request).await {
        CheckResult::BadGateway=> HttpResponse::BadGateway().json("Coludn't get the current user"),
        CheckResult::Unauthorized => {
            println!("{:?}", request.cookie("auth"));
            HttpResponse::Unauthorized().json("Unauthorized")
        },
        CheckResult::Success(user) => {
            match User::get_channels(user.user_id.unwrap(), pool).await {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(e) => HttpResponse::Conflict().json(e.to_string())
            }
        }
    }
}