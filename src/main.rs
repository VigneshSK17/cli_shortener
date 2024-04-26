use std::{net::SocketAddr, time::Duration, str::FromStr};

use axum::{routing, response::{IntoResponse}};
use clap::Parser;
use cli_table::{Cell, CellStruct, Table, Style, print_stdout};
use reqwest::StatusCode;
use sqlx::{sqlite::{SqlitePoolOptions, SqliteConnectOptions}, ConnectOptions};
use utils::{CreateLink, Shortcut};

mod args;
mod controller;
mod db;
mod utils;

// TODO: Fix all unwraps
// TODO: Log everything

#[tokio::main]
async fn main() {

    let args = args::ClapArgs::parse();

    match args.entity_type {
        args::EntityType::Clear => {
            match reqwest::get("http://127.0.0.1:8080/clear").await {
                Err(_) => println!("\nThe links server has not been started. Use the start command to start the server\n"),
                Ok(resp) => {
                    match resp.status() {
                        StatusCode::OK => println!("\nCleared links\n"),
                        _ => println!("\nCould not clear links\n")
                    }
                }
            }
        },
        args::EntityType::Start => {
            init().await;
        },
        args::EntityType::List => {

            if let Ok(resp) = reqwest::get("http://127.0.0.1:8080/all").await {

                if resp.status() == StatusCode::OK {
                    if let Ok(shortcuts) = resp.json::<Vec<Shortcut>>().await {

                        if shortcuts.len() != 0 {
                            let shortcuts_iter = shortcuts.into_iter();

                            let table = shortcuts_iter.map(|s| {
                                vec![s.link.cell(), s.hashed_link.cell()]
                            })
                            .collect::<Vec<Vec<CellStruct>>>()
                            .table()
                            .title(vec!["Original Link".cell().bold(true), "Shortcut Link".cell().bold(true)])
                            .bold(true);

                            if print_stdout(table).is_err() {
                                println!("\nCould not show all shortcut links\n")
                            }
                        } else {
                            println!("\nNo shortcuts have been created yet. Use the new command to create a new link\n")
                        }


                    } else {
                        println!("\nNo links could be found\n")
                    }
                }

            } else {
                println!("\nThe links server has not been started. Use the start command to start the server\n");
            }
        },
        args::EntityType::New(new_command) => {
            // TODO: Add https:// if not in link
            let client = reqwest::Client::new();
            let create_link = CreateLink { link: new_command.link };

            if utils::is_url(&create_link.link) {
                match client.post("http://127.0.0.1:8080/")
                    .json(&create_link)
                    .send().await {
                    Err(_) => println!("\nThe links server has not been started. Use the start command to start the server\n"),
                    Ok(resp) => {
                        match resp.status() {
                            StatusCode::OK => {
                                let hashed_link = resp.text().await.unwrap();
                                println!("\n{} --> {}\n", hashed_link, create_link.link)
                            },
                            _ => println!("\nCould not create shortcut to link\n")
                        }
                    }
                }
            } else {
                println!("\nThe link given is not valid. Make sure to provide the full link address.\n")
            }

        },
        args::EntityType::Delete(delete_command) => {
            let client = reqwest::Client::new();
            let hash = delete_command.link.split('/').last().unwrap();

            match client.delete(format!("http://127.0.0.1:8080/{}", hash))
                .send().await {
                Err(_) => println!("\nThe links server has not been started. Use the start command to start the server\n"),
                Ok(resp) => {
                    match resp.status() {
                        StatusCode::NO_CONTENT => println!("\nDeleted shortcut to link\n"),
                        _ => println!("\nCould not delete given shortcut link\n")
                    }
                }
            }
        }
    }

}

pub async fn init() {

    tracing_subscriber::fmt()
        // TODO: Make it so that if -v then enable
        // .with_max_level(tracing::Level::DEBUG)
        .init();

    let db_url = match db::get_db_path() {
        Ok(s) => s,
        Err(e) => {
            println!("\n{e}\n");
            std::process::exit(1);
        }
    };
    let options = SqliteConnectOptions::from_str(&db_url).unwrap()
        .log_statements(tracing::log::LevelFilter::Debug).clone();

    let pool = match SqlitePoolOptions::new()
        .max_connections(5)
        .idle_timeout(Duration::from_secs(3))
        .connect_with(options)
        .await {

        Ok(p) => p,
        Err(_) => {
            println!("\nCould not connect to database\n");
            std::process::exit(1);
        }

    };

    if db::create_schema(&pool).await.is_err() {
        println!("\nCould not create table inside database\n")
    }

    tracing::debug!("Initialized database at {}", db_url);


    let app = axum::Router::new()
        .route("/", routing::get(test))
        .route("/", routing::post(controller::create_new_link))
        .route("/all", routing::get(controller::get_all_links))
        .route("/clear", routing::get(controller::clear_links))
        .route("/:hash", routing::get(controller::open_link))
        .route("/:hash", routing::delete(controller::delete_link))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    println!("Started on {}", addr);

    if axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.is_err() {
        println!("\nCheck if your local port 8080 is open\n")
    }


}

async fn test() -> impl IntoResponse {
    "Welcome to cli-shortener!"
}