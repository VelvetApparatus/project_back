use actix_web::web::Data;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use uuid::Uuid;

/*
struct for Response to Front
*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    message_id: Option<Uuid>,
    user_id: Option<Uuid>,
    channel_id: Option<Uuid>,
    body: Option<String>,
    is_reply: Option<bool>,
    image: Option<String>,
    created_at: NaiveDateTime,
}


impl Message {
    pub async fn new(
        user_id: Uuid,
        channel_id: Uuid,
        body: String,
        pool: Data<PgPool>
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "Insert into messages Values ($1, $2, $3, $4, null, null, $5)",
            Uuid::new_v4(),
            user_id,
            channel_id,
            body,
            chrono::Local::now().naive_local(),
        )
        .execute(pool.as_ref())
        .await
    }
}
