use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use sqlx::SqlitePool;

use crate::{db, utils};

pub async fn open_link(
    State(pool): State<SqlitePool>,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    match db::get_link(&pool, &hash).await {
        Ok(link) => Redirect::temporary(&link).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not get link for given shortcut",
        )
            .into_response(),
    }
}

pub async fn create_new_link(
    State(pool): State<SqlitePool>,
    extract::Json(create_link): extract::Json<utils::CreateLink>,
) -> impl IntoResponse {
    let link = create_link.link;
    let hash = utils::gen_hash();

    match db::add_link(&pool, &link, &hash).await {
        Ok(_) => format!("http://localhost:8080/{}", hash).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not create shortcut for given link",
        )
            .into_response(),
    }
}

pub async fn get_all_links(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match db::get_all_links(&pool).await {
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not access all shortcuts",
        )
            .into_response(),
        Ok(links) => {
            let shortcuts: Vec<utils::Shortcut> = links
                .iter()
                .map(|(link, hash)| utils::Shortcut {
                    link: link.to_owned(),
                    hashed_link: format!("http://localhost:8080/{}", hash),
                })
                .collect();

            axum::Json(shortcuts).into_response()
        }
    }
}

pub async fn delete_link(
    State(pool): State<SqlitePool>,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    match db::delete_link(&pool, &hash).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not delete shortcut",
        )
            .into_response(),
    }
}

pub async fn clear_links(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match db::clear_links(&pool).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Could not clear links").into_response(),
    }
}
