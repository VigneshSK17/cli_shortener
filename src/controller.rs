use aws_sdk_dynamodb::Client;
use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use crate::{db::{self, Shortcut}, utils};

pub struct DbState {
    client: Client,
    table_name: str
}

pub async fn open_shortcut(
    State((client, table_name)): State<(Client, String)>,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    match db::get_shortcut(&client, &table_name, &hash).await {
        Ok(link) => {
            tracing::info!("Redirected http://localhost:8080/{} to {}", hash, link);
            Redirect::temporary(&link).into_response()
        },
        Err(_) => {
            tracing::error!("Could not get redirect for shortcut http://localhost:8080/{}", hash);
            (StatusCode::INTERNAL_SERVER_ERROR, "Could not get original link for given shortcut")
                .into_response()
        }
    }
}

pub async fn create_new_shortcut(
    State((client, table_name)): State<(Client, String)>,
    extract::Json(create_link): extract::Json<utils::CreateLink>,
) -> impl IntoResponse {
    let shortcut = Shortcut {
        link: create_link.link,
        hash: utils::gen_hash()
    };

    match db::add_shortcut(&client, &table_name, &shortcut).await {
        Ok(_) => {
            tracing::info!("Created shortcut http://localhost:8080/{}", shortcut.hash);
            format!("http://localhost:8080/{}", shortcut.hash).into_response()
        }
        Err(_) => {
            tracing::error!("Could not create shortcut http://localhost:8080/{} from {}", shortcut.hash, shortcut.link);
            (StatusCode::INTERNAL_SERVER_ERROR, "Could not create shortcut for given link")
                .into_response()
        }
    }
}

// pub async fn get_all_links(State((client, table_name)): State<(Client, String)>) -> impl IntoResponse {
//     match db::get_all_links(&pool).await {
//         Err(_) => {
//             tracing::error!("Could not access shortcuts");
//             (StatusCode::INTERNAL_SERVER_ERROR, "Could not access all shortcuts")
//                 .into_response()
//         },
//         Ok(links) => {
//             let shortcuts: Vec<utils::Shortcut> = links
//                 .iter()
//                 .map(|(link, hash)| utils::Shortcut {
//                     link: link.to_owned(),
//                     hashed_link: format!("http://localhost:8080/{}", hash),
//                 })
//                 .collect();

//             tracing::info!("Collected all shortcuts");

//             axum::Json(shortcuts).into_response()
//         }
//     }
// }

pub async fn delete_shortcut(
    State((client, table_name)): State<(Client, String)>,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    match db::delete_shortcut(&client, &table_name, &hash).await {
        Ok(_) => {
            tracing::info!("Deleted http://localhost:8080/{}", hash);
            StatusCode::NO_CONTENT.into_response()
        }
        Err(_) => {
            tracing::error!("Could not delete http://localhost:8080/{}", hash);
            (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete shortcut")
                .into_response()
        }
    }
}

pub async fn clear_shortcuts(State((client, table_name)): State<(Client, String)>) -> impl IntoResponse {
    match db::clear_shortcuts(&client, &table_name).await {
        Ok(_) => {
            tracing::info!("Cleared shortcuts");
            StatusCode::OK.into_response()
        },
        Err(_) => {
            tracing::error!("Could not clear shortcuts");
            (StatusCode::INTERNAL_SERVER_ERROR, "Could not clear shortcuts").into_response()
        }
    }
}