use anyhow::{Ok, Error};
use sqlx::SqlitePool;


#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub token: String,
}

impl User {
    pub fn new(id: i64, token: String) -> Self {
        Self { id, token }
    }

    pub async fn save(&self, sql_conn: &SqlitePool) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO user (id, token) VALUES (?, ?)",
            self.id,
            self.token
        )
        .execute(sql_conn)
        .await?;
        Ok(())
    }
}
