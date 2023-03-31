use std::net::SocketAddr;

use axum::{routing, response::{IntoResponse, Redirect}};


#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let app = axum::Router::new()
        .route("/", routing::get(test));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn test() -> impl IntoResponse {
    "Welcome to cli-shortener!"
    
    // Redirect::permanent("https://www.google.com")
}
