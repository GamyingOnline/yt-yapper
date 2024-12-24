pub mod entities;

use entities::user::User;
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct SqlConn {
    pub sql_conn: SqlitePool,
}

impl SqlConn {
    pub async fn new(sql_url: String) -> Self {
        Self {
            sql_conn: SqlitePool::connect(&sql_url)
                .await
                .expect("Failed to connect to db"),
        }
    }

    pub async fn get_user(&self, id: i64) -> Option<User> {
        Some(
            sqlx::query_as!(User, "SELECT * FROM user where id=(?) LIMIT 1", id)
                .fetch_optional(&self.sql_conn)
                .await
                .expect("User not found")?,
        )
    }
}
