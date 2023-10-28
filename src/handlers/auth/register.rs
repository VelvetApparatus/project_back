use actix_web::{web::{Json, Data}, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::chat::user::User, utils::password_hashing};



#[derive(Deserialize)]
pub struct RegistrationData {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn reg(
    invitation_data: Json<RegistrationData>,
    pool: Data<PgPool>
) -> HttpResponse {
    let mock = invitation_data.into_inner();
    let id = Uuid::new_v4();
    let password = password_hashing::hash(mock.password.as_bytes(), id.clone().to_string());


    if User::get_by_email(&mock.email, &pool).await.unwrap().len() > 0 {
        return HttpResponse::BadRequest().json("Email must be unique")
    }
    match User::create(
        id,
        mock.email,
        password.to_string(),
        mock.username,
        &pool
    ).await {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(_) => {
            HttpResponse::Conflict().json("Err with database")
        }
    }
}