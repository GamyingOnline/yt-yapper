pub mod entities;

use anyhow::Result;
use sqlx::SqlitePool;

pub async fn connect() -> Result<SqlitePool> {
    let conn_uri = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    Ok(SqlitePool::connect(&conn_uri).await?)
}
