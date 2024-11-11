#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub api_key: String,
    pub access_secret: String,
}

impl User {
    pub fn new(id: i64, api_key: String, access_secret: String) -> Self {
        Self {
            id,
            api_key,
            access_secret,
        }
    }
}
