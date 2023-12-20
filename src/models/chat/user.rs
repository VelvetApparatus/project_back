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
    pub last_online: Option<chrono::NaiveDateTime>,
    pub icon: Option<String>,
    pub background: Option<String>,    

}

#[derive(Serialize)]
pub struct SearchUser {
    pub out_user_id: Option<Uuid>,
    pub out_username: Option<String>,
    pub out_image: Option<String>,
    pub out_is_online: Option<bool>,
    pub out_last_online: Option<chrono::NaiveDateTime>,
    pub out_icon: Option<String>,
    pub out_background: Option<String>,    
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct StructForGetChannels {
    channel_id: Option<Uuid>,
    channel_name: Option<String>,
    last_message_id: Option<Uuid>,
    last_message_text: Option<String>,
    last_message_timestamp: Option<chrono::NaiveDateTime>,
    channel_img: Option<String>
}

/*
    channel_id UUID,
    channel_name TEXT,
    last_message_id UUID,
    last_message_text TEXT,
    last_message_timestamp TIMESTAMP,
    channel_img TEXT
*/
impl User {
    
    
    pub async fn get_channels(
        user_id: Uuid,
        pool: Data<PgPool>
    ) -> Result<Vec<StructForGetChannels>, sqlx::Error> {
        sqlx::query_as!(
            StructForGetChannels, 
            "SELECT * FROM get_channel_data($1)",
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
                username,
                login,
                password_hash,
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


    pub async fn get_by_user_id(
        user_id: &Uuid,
        pool: &Data<PgPool>
    ) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "
            SELECT * FROM users WHERE user_id = $1
            ", 
            user_id
        )
        .fetch_all(pool.as_ref())
        .await
    }


    pub async fn join_channel(
        channels: &Vec<Uuid>,
        user_id: &Uuid,
        pool: &Data<PgPool>
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "
            UPDATE users set channels = $1 WHERE user_id = $2 
            ",
            channels,
            user_id
        )
        .execute(pool.as_ref())
        .await
    }


    pub async fn insert_channel(
        channel_id: &Vec<Uuid>,
        user_id: &Uuid,
        pool: &Data<PgPool>
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "UPDATE users
            SET channels = channels || $1
            WHERE user_id = $2;",
            channel_id,
            user_id
        ).execute(pool.as_ref()).await
    }


    pub async fn update_user(
        user_id: &Uuid,
        icon: &String,
        background: &String,
        username: &String,
        pool: &Data<PgPool>
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "
            UPDATE users SET icon = $1, background = $2, username = $3  WHERE user_id = $4
            ",
            icon,
            background,
            username,
            user_id
        )
        .execute(pool.as_ref())
        .await
    }

    pub async fn search(
        search_text: &str,
        pool: &Data<PgPool>   
    ) -> Result<Vec<SearchUser>, sqlx::Error> {
        sqlx::query_as!(
            SearchUser,
            "SELECT * FROM search_users($1)",
            search_text
        )
        .fetch_all(pool.as_ref())
        .await

        
    }
}   