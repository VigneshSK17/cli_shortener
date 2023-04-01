use std::{fs::File, path::PathBuf, env};

use sqlx::SqlitePool;


pub fn get_db_path() -> (PathBuf, String) {

    let mut path = dirs::data_local_dir()
        .unwrap_or(env::current_dir().expect("Could not find a directory to store links"));

    path.push("cli_shortener/links.db");

    if !path.is_file() {
        File::create(&path).expect("Could not create database to store links");
    }

    tracing::info!("{:?}", path);

    let path_str = path.to_string_lossy().into_owned();

    (path, path_str)

}

// pub async fn create_db(path: &PathBuf) {
//
//     let path_str = path.to_string_lossy();
//
//     if !Sqlite::database_exists(&path_str).await.unwrap_or(false) {
//
//         Sqlite::create_database(&path_str).await
//             .expect("Could not create database to store links");
//
//         create_schema(&path_str).await;
//
//     }
//
//     tracing::info!("Database already exists at {}", path_str);
//
// } 

pub async fn create_schema(pool: &SqlitePool) {

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS links (
            link text,
            hash text
        );
    "#)
        .execute(pool)
        .await
        .expect("Could not create table inside database");

    // tracing::info!("Initialized database at {}");


}
