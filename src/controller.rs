use std::net::SocketAddr;

use askama::Template;
use aws_sdk_dynamodb::Client;
use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};

use crate::{
    db::{self, Shortcut},
    utils::{self, is_url, IndexTemplate},
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

    if !is_url(&create_link.link) {
        tracing::error!(
            "Could not verify that the provided link is a valid URL: {}",
            create_link.link
        );
        return (
            StatusCode::BAD_REQUEST,
            "Invalid URL provided as link"
        ).into_response();
    }

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

    if let Err(e) = db::get_shortcut(&client, &table_name, &hash).await {
        tracing::error!("Could not locate shortcut with {hash}: {e:?}");
        return (
            StatusCode::BAD_REQUEST,
            "The given shortcut does not exist",
        )
            .into_response()
    }

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

pub async fn index(
    State((client, table_name, _)): State<(Client, String, SocketAddr)>,
) -> impl IntoResponse {
    let template = IndexTemplate { url: "/".to_string() };

    match template.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()).into_response(),
        Err(e) => {
            tracing::error!("Could not render template: {e:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Could not render template").into_response()
        }
    }
}