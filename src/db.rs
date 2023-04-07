use std::{env, fs::File};

use sqlx::{Row, SqlitePool};

pub fn get_db_path() -> Result<String, String> {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| env::current_dir().expect("Could not find a directory to store links"));

    path.push("cli_shortener/links.db");

    if !path.is_file() && File::create(&path).is_err() {
        return Err("Could not create database to store links".to_string());
    }

    tracing::debug!("{:?}", path);

    Ok(path.to_string_lossy().to_string())
}

pub async fn create_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS links (
            link text,
            hash text
        );
    "#,
    )
    .execute(pool)
    .await?;

    Ok(())
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

pub async fn get_all_links(pool: &SqlitePool) -> Result<Vec<(String, String)>, sqlx::Error> {
    let rows = sqlx::query("SELECT * from links").fetch_all(pool).await?;

    let links: Vec<(String, String)> = rows
        .iter()
        .map(|r| (r.get("link"), r.get("hash")))
        .collect();

    Ok(links)
}

pub async fn delete_link(pool: &SqlitePool, hash: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE from links WHERE hash = $1")
        .bind(hash)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn clear_links(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE from links").execute(pool).await?;

    Ok(())
}
