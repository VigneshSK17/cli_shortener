use axum::{response::{IntoResponse, Redirect}, extract::{Path, State}, http::StatusCode};
use sqlx::SqlitePool;

use crate::db;


pub async fn open_link(
    State(pool): State<SqlitePool>,
    Path(hash): Path<String>
) -> impl IntoResponse {

    match db::get_link(&pool, &hash).await {
        Ok(link) => Redirect::temporary(&link).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Could not get link for given shortcut").into_response()
    }

}

pub async fn clear_links(
    State(pool): State<SqlitePool>
) -> impl IntoResponse {

    match db::clear_links(&pool).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Could not clear links").into_response()
    }

}
