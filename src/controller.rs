use axum::{response::{IntoResponse, Redirect}, extract::{Path, State, self}, http::StatusCode};
use sqlx::SqlitePool;

use crate::{db, utils};

pub async fn open_link(
    State(pool): State<SqlitePool>,
    Path(hash): Path<String>
) -> impl IntoResponse {

    match db::get_link(&pool, &hash).await {
        Ok(link) => Redirect::temporary(&link).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Could not get link for given shortcut").into_response()
    }

}

pub async fn create_new_link(
    State(pool): State<SqlitePool>,
    extract::Json(create_link): extract::Json<utils::CreateLink>
) -> impl IntoResponse {

    let link = create_link.link;
    let hash = utils::gen_hash();

    if let Err(_) = db::add_link(&pool, &link, &hash).await {
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not create shortcut for given link").into_response()
    } else {
        format!("http://localhost:8080/{}", hash).into_response()
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
