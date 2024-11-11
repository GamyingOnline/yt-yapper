use anyhow::Result;
use sqlx::SqlitePool;

pub async fn connect() -> Result<SqlitePool> {
    Ok(SqlitePool::connect("sqlite::memory:").await?)
}
