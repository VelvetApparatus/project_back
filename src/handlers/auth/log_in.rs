use actix_identity::Identity;
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::auth::user::AuthorizedUser, utils::password_hashing::hash};



#[derive(Deserialize, Serialize)]
pub struct AuthorizationData {
    pub email: String,
    pub password: String,
}


pub async fn simple_log_in(
    req: Json<Option<AuthorizationData>>,
    pool: Data<PgPool>,
    id: Identity
) -> impl Responder {
    let mock = req.into_inner().unwrap();

    let users = AuthorizedUser::get_by_email(&mock.email, &pool).await;

    match users {
        Ok(value) => {
            match value.first() {
                Some(user) => {
                    let hash_password = hash(mock.password.as_bytes(), user.id.to_string()).to_string();

                    match hash_password.eq(&user.password_hash) {
                        true => {
                            id.remember(Uuid::new_v4().to_string());
                            HttpResponse::Ok().finish()
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