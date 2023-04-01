use std::{net::SocketAddr, time::Duration};

use axum::{routing, response::{IntoResponse, Redirect}, extract::State};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

mod db;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let (db_url, db_url_str) = db::get_db_path();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .idle_timeout(Duration::from_secs(3))
        .connect(&db_url_str)
        .await
        .expect("Could not connect to database");

    db::create_schema(&pool)
        .await;

    tracing::info!("Initialized database at {}", db_url_str);


    let app = axum::Router::new()
        .route("/", routing::get(test_db))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn test() -> impl IntoResponse {
    let path = db::get_db_path();

    "Welcome to cli-shortener!"
    
    // Redirect::permanent("https://www.google.com")

}

async fn test_db(
    State(pool): State<SqlitePool>
) -> impl IntoResponse {

    sqlx::query("INSERT INTO links (link, hash) values ($1, $2)")
        .bind("abc")
        .bind("def")
        .execute(&pool)
        .await
        .unwrap();

    "Hi"
}
