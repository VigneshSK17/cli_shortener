use std::{
    env,
    fs::File,
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use args::ClapArgs;
use axum::routing;
use clap::Parser;
use cli_table::{print_stdout, Cell, CellStruct, Style, Table};
use dotenv::dotenv;
use reqwest::StatusCode;

mod args;
mod controller;
mod db;
mod utils;

#[tokio::main]
async fn main() {
    let args = args::ClapArgs::parse();

    let (local_addr, path) = match matches!(args.entity_type, args::EntityType::Start) {
        false => match get_local_addr() {
            Ok(addr) => addr,
            Err(_) => {
                println!("Could not get address to access cli_shortener");
                return;
            }
        },
        true => (SocketAddr::from(([127, 0, 0, 1], 0)), "".to_string()), // just to initialize the variable
    };

    match args.entity_type {
        args::EntityType::Start => {
            init(args).await;
        }
        args::EntityType::List => {
            if let Ok(resp) = reqwest::get(format!("http://{local_addr}/{path}/all")).await {
                if resp.status() == StatusCode::OK {
                    if let Ok(shortcuts) = resp.json::<Vec<db::Shortcut>>().await {
                        if !shortcuts.is_empty() {
                            let shortcuts_iter = shortcuts.into_iter();

                            let table = shortcuts_iter
                                .map(|s| vec![s.link.cell(), s.hash.cell()])
                                .collect::<Vec<Vec<CellStruct>>>()
                                .table()
                                .title(vec![
                                    "Original Link".cell().bold(true),
                                    "Shortcut Link".cell().bold(true),
                                ])
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
        }
        args::EntityType::New(new_command) => {
            let client = reqwest::Client::new();
            let create_link = utils::CreateLink {
                link: new_command.link,
            };

            if utils::is_url(&create_link.link) {
                match client.post(format!("http://{local_addr}/{path}"))
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
                println!(
                    "\nThe link given is not valid. Make sure to provide the full link address."
                )
            }
        }
        args::EntityType::Delete(delete_command) => {
            let client = reqwest::Client::new();
            let hash = delete_command.link.split('/').last().unwrap();

            let parent_path= match path.as_str() {
                "" => "".to_string(),
                p => format!("{p}/")
            };

            match client.delete(format!("http://{local_addr}/{parent_path}{hash}"))
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
        .with_max_level(if args.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .compact()
        .init();

    let addr = gen_addr(&args);
    let path = args.path.as_str();
    let parent_path= match args.path.as_str() {
        "" => "".to_string(),
        p => format!("{p}/")
    };


    let app = axum::Router::new()
        .route(&format!("/{path}"), routing::get(controller::index))
        .route(&format!("/{path}"), routing::post(controller::create_new_shortcut))
        .route(&format!("/{parent_path}all"), routing::get(controller::get_all_shortcuts))
        .route(&format!("/{parent_path}:hash"), routing::get(controller::open_shortcut))
        .route(&format!("/{parent_path}:hash"), routing::delete(controller::delete_shortcut))
        .with_state((db_client, db_table_name, addr, path.to_string()));

    let binding = axum::Server::try_bind(&addr);

    match binding {
        Err(_) => tracing::error!("Cannot attach server to address {}", addr),
        Ok(b) => {
            let server = b.serve(app.into_make_service());
            let local_addr = server.local_addr();

            tracing::info!("Started on: http://{local_addr}/{path}");

            if store_local_addr(&local_addr, &args.path).is_err() {
                tracing::error!("Could not store local address of server");
            }

            if server.await.is_err() {
                tracing::error!("Server stopped unexpectedly");
            }
        }
    }
}

fn gen_addr(args: &ClapArgs) -> SocketAddr {
    let mut addr = SocketAddr::from((
        args.host
            .parse::<IpAddr>()
            .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)),
        args.port,
    ));
    let addr_str = format!("{}:{}", args.host, args.port);
    let listener = std::net::TcpListener::bind(&addr_str);

    if listener.is_err() {
        tracing::error!(
            "Cannot bind to address {}, generating random port",
            addr_str
        );
        addr.set_port(0);
    }
    std::mem::drop(listener);

    addr
}

fn store_local_addr(addr: &SocketAddr, path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create("/tmp/cli_shortener.txt")?;
    file.write_all(format!("{}\n{}\n{}", &addr.ip(), &addr.port(), &path).as_bytes())?;
    Ok(())
}

fn get_local_addr() -> Result<(SocketAddr, String), std::io::Error> {
    let mut file = File::open("/tmp/cli_shortener.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let values = contents.split_whitespace().collect::<Vec<&str>>();

    let host = values[0]
        .parse::<IpAddr>()
        .map_err(|_| std::io::Error::last_os_error())?;
    let port = values[1]
        .parse::<u16>()
        .map_err(|_| std::io::Error::last_os_error())?;
    let path = values[2]
        .parse::<String>()
        .map_err(|_| std::io::Error::last_os_error())?;


    Ok((SocketAddr::from((host, port)), path))
}
