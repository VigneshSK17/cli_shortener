use std::net::SocketAddr;

use axum::routing;



#[tokio::main]
async fn main() {

    let app = axum::Router::new()
        .route("/", routing::get(test));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn test() -> &'static str {
    "Welcome to cli-shortener!"
}
