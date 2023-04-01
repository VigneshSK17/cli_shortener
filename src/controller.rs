use axum::{response::{IntoResponse, Redirect}, extract::{Path, State}};
use sqlx::SqlitePool;

use crate::db;


pub async fn open_link(
    State(pool): State<SqlitePool>,
    Path(hash): Path<String>
) -> impl IntoResponse {

    let link = db::get_link(&pool, &hash).await
        .expect("Could not get link for given shortcut");

    Redirect::permanent(&link)
    // Redirect::permanent("https://www.google.com")

}
