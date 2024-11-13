use std::{collections::HashMap, sync::mpsc::Sender};

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use sqlx::SqlitePool;

use crate::persistence::entities::user::User;

pub fn server(sqlite_conn: SqlitePool, tx: Sender<User>) -> Router {
    axum::Router::new()
        .route("/callback", get(callback_handler))
        .with_state(ServerState { sqlite_conn, tx })
}

#[derive(Clone)]
struct ServerState {
    sqlite_conn: SqlitePool,
    tx: Sender<User>,
}

#[axum::debug_handler]
pub async fn callback_handler(
    Query(query): Query<HashMap<String, String>>,
    State(server_state): State<ServerState>,
) -> Json<HashMap<String, String>> {
    let mut conn = server_state.sqlite_conn.acquire().await.unwrap();
    let user_id = query.get("user_id").unwrap();
    let token = query.get("token").unwrap();
    sqlx::query!(
        r#"
            INSERT INTO users values ( ?1, ?2 )
        "#,
        user_id,
        token
    )
    .execute(&mut *conn);
    server_state
        .tx
        .send(User::new(user_id.parse().unwrap(), token.clone()));
    Json(HashMap::new())
}
