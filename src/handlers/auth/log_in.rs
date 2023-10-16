use actix_web::{web::{Json, Data}, HttpResponse, Responder, cookie::Cookie};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::chat::user::User, utils::password_hashing::hash};



#[derive(Deserialize, Serialize)]
pub struct AuthorizationData {
    pub email: String,
    pub password: String,
}


pub async fn log_in(
    req: Json<Option<AuthorizationData>>,
    pool: Data<PgPool>,
) -> impl Responder {
    let mock = req.into_inner().unwrap();

    let users = User::get_by_email(&mock.email, &pool).await;

    match users {
        Ok(value) => {
            match value.first() {
                Some(user) => {
                    // Checking that the password is correct
                    let hash_password = hash(mock.password.as_bytes(), user.user_id.unwrap().to_string()).to_string();
                    let old_password_hash = user.password_hash.clone().unwrap();
                    match hash_password.eq(&old_password_hash) {
                        true => {

                            let new_uuid = user.user_id.unwrap();

                            // Sending the Response
                            HttpResponse::Ok()
                                .cookie(Cookie::new("auth", new_uuid.to_string()))
                                .json(new_uuid.to_string())

                        },
                        false => {HttpResponse::Unauthorized().json("Password are invalid")}
                    }
                },
                None => {HttpResponse::BadRequest().json("Current user does not exist")}
            }
        },
        Err(_) => {HttpResponse::BadGateway().json("Problem with database")}
    }
}