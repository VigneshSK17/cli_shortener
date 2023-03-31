use std::{path::PathBuf, env};


pub fn get_db_path() {

    let mut path = dirs::data_local_dir()
        .unwrap_or(env::current_dir().expect("Could not find a directory to store links"));

    path.push("cli_shortener/links.db");

    tracing::info!("{:?}", path);

}
