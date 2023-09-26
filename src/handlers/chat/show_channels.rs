use actix_identity::Identity;
use actix_web::{web::Data, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::chat::user::User;



pub async fn show_channels(
    pool: Data<PgPool>,
    id: Identity
) -> HttpResponse {
    if let Some(id) = id.identity() {
        match Uuid::parse_str(&id) {
            Ok(value) => {
                let channels = User::get_channels(value, pool).await;
                if channels.is_err() {
                    return HttpResponse::BadRequest().finish()
                }
                HttpResponse::Ok().json(channels.unwrap())
            },
            Err(_) => HttpResponse::Unauthorized().finish()
        }        
    } else {
        HttpResponse::Unauthorized().finish()
    }
}