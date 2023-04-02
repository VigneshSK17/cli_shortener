use std::{net::SocketAddr, time::Duration};

use axum::{routing, response::{IntoResponse, Redirect}, extract::State};
use clap::Parser;
use reqwest::StatusCode;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

mod args;
mod controller;
mod db;
mod utils;

#[tokio::main]
async fn main() {

    let args = args::ClapArgs::parse();

    match args.entity_type {
        args::EntityType::Clear => {
            match reqwest::get("http://127.0.0.1:8080/clear").await {
                Err(_) => println!("The links server has not been started. Use the start command to start the server"),
                Ok(resp) => {
                    match resp.status() {
                        StatusCode::OK => println!("Cleared links"),
                        _ => println!("Could not clear links")
                    }
                }
            }
        },
        args::EntityType::Start => {
            init().await
        },
        args::EntityType::List => {
            todo!("Use a cli table library")
        },
        args::EntityType::New(new_command) => {
            // db::add_link(, link, hash)
        },
        args::EntityType::Delete(delete_command) => {
            todo!()
        }
    }

}

async fn init() {

    tracing_subscriber::fmt::init();

    let db_url = db::get_db_path();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .idle_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .expect("Could not connect to database");

    db::create_schema(&pool)
        .await;

    tracing::info!("Initialized database at {}", db_url);


    let app = axum::Router::new()
        .route("/", routing::get(test_db))
        .route("/clear", routing::get(controller::clear_links))
        .route("/:hash", routing::get(controller::open_link))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test() -> impl IntoResponse {
    // "Welcome to cli-shortener!"
    
    // Redirect::permanent("https://www.google.com")

    utils::gen_hash()
}

async fn test_db(
    State(pool): State<SqlitePool>
) -> impl IntoResponse {

    match db::add_link(&pool, "https://www.google.com", "gg").await {
        Ok(_) => tracing::info!("Created link http://localhost:8080/{} for link {}", "klm", "hij"),
        Err(_) => tracing::error!("Could not create shortened link for {}", "hij")
    }

    "Hi"
}
