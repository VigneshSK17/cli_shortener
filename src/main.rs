use std::{env, net::SocketAddr, time::Duration};

use args::ClapArgs;
use axum::{routing, response::IntoResponse};
use clap::Parser;
use cli_table::{Cell, CellStruct, Table, Style, print_stdout};
use controller::DbState;
use dotenv::dotenv;
use reqwest::StatusCode;
use utils::{CreateLink, Shortcut};

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
                Err(_) => println!("\nThe links server has not been started. Use the start command to start the server"),
                Ok(resp) => {
                    match resp.status() {
                        StatusCode::OK => println!("\nCleared links"),
                        _ => println!("\nCould not clear links")
                    }
                }
            }
        },
        args::EntityType::Start => {
            init(args).await;
        },
        args::EntityType::List => {

            if let Ok(resp) = reqwest::get("http://127.0.0.1:8080/all").await {

                if resp.status() == StatusCode::OK {
                    if let Ok(shortcuts) = resp.json::<Vec<Shortcut>>().await {

                        if !shortcuts.is_empty() {
                            let shortcuts_iter = shortcuts.into_iter();

                            let table = shortcuts_iter.map(|s| {
                                vec![s.link.cell(), s.hashed_link.cell()]
                            })
                            .collect::<Vec<Vec<CellStruct>>>()
                            .table()
                            .title(vec!["Original Link".cell().bold(true), "Shortcut Link".cell().bold(true)])
                            .bold(true);

                            if print_stdout(table).is_err() {
                                println!("\nCould not show all shortcut links")
                            }
                        } else {
                            println!("\nNo shortcuts have been created yet. Use the new command to create a new link")
                        }


                    } else {
                        println!("\nNo links could be found")
                    }
                }

            } else {
                println!("\nThe links server has not been started. Use the start command to start the server");
            }
        },
        args::EntityType::New(new_command) => {
            let client = reqwest::Client::new();
            let create_link = CreateLink { link: new_command.link };

            if utils::is_url(&create_link.link) {
                match client.post("http://127.0.0.1:8080/")
                    .json(&create_link)
                    .send().await {
                    Err(_) => println!("\nThe links server has not been started. Use the start command to start the server"),
                    Ok(resp) => {
                        match resp.status() {
                            StatusCode::OK => {
                                let hashed_link = resp.text().await.unwrap();
                                println!("\n{} --> {}", hashed_link, create_link.link)
                            },
                            _ => println!("\nCould not create shortcut to link")
                        }
                    }
                }
            } else {
                println!("\nThe link given is not valid. Make sure to provide the full link address.")
            }

        },
        args::EntityType::Delete(delete_command) => {
            let client = reqwest::Client::new();
            let hash = delete_command.link.split('/').last().unwrap();

            match client.delete(format!("http://127.0.0.1:8080/{}", hash))
                .send().await {
                Err(_) => println!("\nThe links server has not been started. Use the start command to start the server"),
                Ok(resp) => {
                    match resp.status() {
                        StatusCode::NO_CONTENT => println!("\nDeleted shortcut to link"),
                        _ => println!("\nCould not delete given shortcut link")
                    }
                }
            }
        }
    }

}

pub async fn init(args: ClapArgs) {

    dotenv().ok();
    let db_client = db::init_db_client().await;
    let db_table_name = env::var("AWS_TABLE_NAME").unwrap();

    tracing_subscriber::fmt()
        .with_max_level(
            if args.verbose { tracing::Level::DEBUG } else { tracing::Level::INFO }
        )
        .compact()
        .init();

    let app = axum::Router::new()
        .route("/s", routing::get(test))
        .route("/s", routing::post(controller::create_new_shortcut))
        // .route("/s/all", routing::get(controller::get_all_links))
        .route("/s/clear", routing::get(controller::clear_shortcuts))
        .route("/s/:hash", routing::get(controller::open_shortcut))
        .route("/s/:hash", routing::delete(controller::delete_shortcut))
        .with_state((db_client, db_table_name));

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));

    let binding = axum::Server::try_bind(&addr);
    match binding {
        Err(_) => tracing::error!("Cannot attach server to port 8080"),
        Ok(b) => {
            let server = b.serve(app.into_make_service());
            let local_addr = server.local_addr();

            tracing::info!("Started on: http://localhost:{}", local_addr.port());
            if server.await.is_err() {
                tracing::error!("Check if your local ports are open")
            }
        }

    }



}

async fn test() -> impl IntoResponse {
    "Welcome to cli-shortener!"
}