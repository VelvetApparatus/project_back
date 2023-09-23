use actix_web::web::Data;
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use uuid::Uuid;




#[derive(sqlx::FromRow)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct AuthorizedUser {
    pub id: Uuid,
    pub login: String,
    pub password_hash: String,
    pub username: String,
}


impl AuthorizedUser {

    pub async fn create(
        &self,
        pool: &Data<PgPool>
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
            sqlx::query!(
                "
                INSERT INTO authorized_users VALUES
                ($1, $2, $3, $4)
                ",
                self.id,
                self.login,
                self.password_hash,
                self.username
            )
                .execute(pool.as_ref())
                .await
    }


    pub async fn get_by_email(
        login: &String,
        pool: &Data<PgPool>
    ) -> Result<Vec<AuthorizedUser>, sqlx::Error> {
        sqlx::query_as!(
            AuthorizedUser,
            "
            SELECT * FROM authorized_users WHERE login = $1
            ", 
            login
        )
        .fetch_all(pool.as_ref())
        .await
    }
}

