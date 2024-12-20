#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub token: String,
}

impl User {
    pub fn new(id: i64, token: String) -> Self {
        Self { id, token }
    }
}
