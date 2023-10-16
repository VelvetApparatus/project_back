use actix_web::{HttpResponse, cookie::{Cookie, time::Duration}};



pub async fn log_out() -> HttpResponse {

    HttpResponse::Ok()
    .cookie(
        Cookie::build("auth", "")
        .max_age(Duration::ZERO)
        .finish()
    )
    .json("Logged out successfully")
}