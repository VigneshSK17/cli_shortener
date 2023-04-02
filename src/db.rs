use std::{fs::File, env};

use sqlx::{Row, SqlitePool};


pub fn get_db_path() -> String {

    let mut path = dirs::data_local_dir()
        .unwrap_or(env::current_dir().expect("Could not find a directory to store links"));

    path.push("cli_shortener/links.db");

    if !path.is_file() {
        File::create(&path).expect("Could not create database to store links");
    }

    tracing::info!("{:?}", path);

    path.to_string_lossy().into_owned()

}

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


}

pub async fn add_link(pool: &SqlitePool, link: &str, hash: &str) -> Result<(), sqlx::Error> {

    sqlx::query("INSERT INTO links (link, hash) values ($1, $2)")
        .bind(link)
        .bind(hash)
        .execute(pool)
        .await?;

    Ok(())

}

pub async fn get_link(pool: &SqlitePool, hash: &str) -> Result<String, sqlx::Error> {

    let row = sqlx::query("SELECT link from links WHERE hash = $1")
        .bind(hash)
        .fetch_one(pool)
        .await?;

    Ok(row.get("link"))
}

pub async fn clear_links(pool: &SqlitePool) -> Result<(), sqlx::Error> {

    sqlx::query("DELETE from links")
        .execute(pool)
        .await?;

    Ok(())

}
