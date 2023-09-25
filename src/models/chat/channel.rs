use std::sync::Arc;
use actix_web::web::Data;
use actix_web_lab::sse::{Sse, ChannelStream};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::{user::User, broadcast::Broadcaster};

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub users: Vec<User>,
    pub broadcaster: Arc<Broadcaster>,
}

impl Channel {

    pub fn get_users(&self) {
        let mut result = String::from("Users in channel: ");
        for user in self.users.iter() {
            result.push_str(&format!("{}, ", user.name));
        }
        result.pop();
        result.pop();
    }

    pub async fn add_user(&mut self, user: &User) -> Sse<ChannelStream> {
        self.users.push(user.clone());
        self.broadcaster.new_client().await
        
    }

    pub fn remove_user(&mut self, user: &User) {
        self.users = self.users.iter().filter(|&u| u.name != user.name).cloned().collect();
    }

    pub async fn message(&self, message: String) {
        // for user in self.users.iter() {
            self.broadcaster.broadcast(message.as_str()).await
        // }
    }

}


#[derive(sqlx::FromRow)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChannelDB {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub users: Option<Vec<Uuid>>,
}

impl ChannelDB {
    pub async fn new(
        name: String,
        user: Uuid,
        pool: Data<PgPool>
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO channels VALUES ($1, $2, $3)",
            Uuid::new_v4(),
            name,
            &[user]
        )
        .execute(pool.as_ref())
        .await
    }


    pub async fn get_by_id(
        id: Uuid,
        pool: Data<PgPool>
    ) -> Result<Vec<ChannelDB>, sqlx::Error> {
        sqlx::query_as!(
            ChannelDB,
            "SELECT * FROM channels WHERE id = $1",
            id
        )
        .fetch_all(pool.as_ref())
        .await
        
    }


}

