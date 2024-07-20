use std::net::SocketAddr;

use aws_sdk_dynamodb::Client;
use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use crate::{
    db::{self, Shortcut},
    utils,
};

pub async fn open_shortcut(
    State((client, table_name, address)): State<(Client, String, SocketAddr)>,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    match db::get_shortcut(&client, &table_name, &hash).await {
        Ok(link) => {
            tracing::info!("Redirected http://{address}/{hash} to {link}");
            Redirect::temporary(&link).into_response()
        }
        Err(e) => {
            tracing::error!("Could not get redirect for shortcut with {hash}: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not get original link for given shortcut",
            )
                .into_response()
        }
    }
}

pub async fn create_new_shortcut(
    State((client, table_name, address)): State<(Client, String, SocketAddr)>,
    extract::Json(create_link): extract::Json<utils::CreateLink>,
) -> impl IntoResponse {
    let shortcut = Shortcut {
        link: create_link.link,
        hash: utils::gen_hash(),
    };

    match db::add_shortcut(&client, &table_name, &shortcut).await {
        Ok(_) => {
            tracing::info!("Created shortcut http://{address}/{}", shortcut.hash);
            format!("http://{address}/{}", shortcut.hash).into_response()
        }
        Err(e) => {
            tracing::error!(
                "Could not create shortcut with hash {} from {}: {e:?}",
                shortcut.hash,
                shortcut.link
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not create shortcut for given link",
            )
                .into_response()
        }
    }
}

pub async fn get_all_shortcuts(
    State((client, table_name, _)): State<(Client, String, SocketAddr)>,
) -> impl IntoResponse {
    match db::get_all_shortcuts(&client, &table_name).await {
        Ok(items) => {
            let shortcuts: Vec<Shortcut> = items
                .iter()
                .filter_map(|fields| {
                    if (fields.get("link").is_none() || fields.get("link").unwrap().as_s().is_err())
                        || (fields.get("link_hash").is_none()
                            || fields.get("link_hash").unwrap().as_s().is_err())
                    {
                        return None;
                    }
                    Some(Shortcut {
                        link: fields.get("link").unwrap().as_s().unwrap().to_string(),
                        hash: fields.get("link_hash").unwrap().as_s().unwrap().to_string(),
                    })
                })
                .collect();

            tracing::info!("Collected all shortcuts");

            axum::Json(shortcuts).into_response()
        }
        Err(e) => {
            tracing::error!("Could not access shortcuts: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not access all shortcuts",
            )
                .into_response()
        }
    }
}

pub async fn delete_shortcut(
    State((client, table_name, _)): State<(Client, String, SocketAddr)>,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    match db::delete_shortcut(&client, &table_name, &hash).await {
        Ok(_) => {
            tracing::info!("Deleted shortcut with hash {hash}");
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            tracing::error!("Could not delete shortcut with hash {hash}: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not delete shortcut",
            )
                .into_response()
        }
    }
}

pub async fn clear_shortcuts(
    State((client, table_name, _)): State<(Client, String, SocketAddr)>,
) -> impl IntoResponse {
    match db::clear_shortcuts(&client, &table_name).await {
        Ok(_) => {
            tracing::info!("Cleared shortcuts");
            StatusCode::OK.into_response()
        }
        Err(e) => {
            tracing::error!("Could not clear shortcuts: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not clear shortcuts",
            )
                .into_response()
        }
    }
}
