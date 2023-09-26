use actix_web::web::Data;
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub login: Option<String>,
    pub password_hash: Option<String>,
    pub channels: Option<Vec<Uuid>>,
    pub image: Option<String>,
    pub is_online: Option<bool>,
    pub last_online: Option<chrono::NaiveDateTime>

}
/*
-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY,
    username TEXT NOT NULL,
    login TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    channels UUID[],
    Image TEXT,
    is_online BOOLEAN,
    last_online TIMESTAMP
);

 */

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct StructForGetChannels {
    channel_name: Option<String>,
}
impl User {
    
    
    pub async fn get_channels(
        user_id: Uuid,
        pool: Data<PgPool>
    ) -> Result<Vec<StructForGetChannels>, sqlx::Error> {
        sqlx::query_as!(
            StructForGetChannels,
            "SELECT * FROM get_user_channels($1)",
            user_id
        )
        .fetch_all(pool.as_ref())
        .await
    }


    pub async fn create(
        user_id: Uuid,
        login: String,
        password_hash: String,
        username: String,
        pool: &Data<PgPool>
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
            sqlx::query!(
                "
                INSERT INTO users VALUES
                ($1, $2, $3, $4)
                ",
                user_id,
                login,
                password_hash,
                username
            )
                .execute(pool.as_ref())
                .await
    }


    pub async fn get_by_email(
        login: &String,
        pool: &Data<PgPool>
    ) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "
            SELECT * FROM users WHERE login = $1
            ", 
            login
        )
        .fetch_all(pool.as_ref())
        .await
    }
}