use std::fs::File;

use sqlx::{Row, SqlitePool};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbPathError {
    #[error("directory not found")]
    DataDirError(),

    #[error("database not created")]
    DbFileCreationError(#[from] std::io::Error),

}

/// Grabs path for links database, creates if necessary
pub fn get_db_path() -> Result<String, DbPathError> {
    let mut path = dirs::data_local_dir()
        .ok_or(DbPathError::DataDirError())?;

    path.push("cli_shortener/links.db");

    if !path.is_file() {
        File::create(&path)?;
    }

    Ok(path.to_string_lossy().to_string())
}

/// Genreates schema for creating new links db
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

/// Add link with generated hash to links db
pub async fn add_link(pool: &SqlitePool, link: &str, hash: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO links (link, hash) values ($1, $2)")
        .bind(link)
        .bind(hash)
        .execute(pool)
        .await?;

    Ok(())
}

/// Get the given hash's link from links db
pub async fn get_link(pool: &SqlitePool, hash: &str) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT link from links WHERE hash = $1")
        .bind(hash)
        .fetch_one(pool)
        .await?;

    Ok(row.get("link"))
}

/// Get all links from links db
pub async fn get_all_links(pool: &SqlitePool) -> Result<Vec<(String, String)>, sqlx::Error> {
    let rows = sqlx::query("SELECT * from links").fetch_all(pool).await?;

    let links: Vec<(String, String)> = rows
        .iter()
        .map(|r| (r.get("link"), r.get("hash")))
        .collect();

    Ok(links)
}

/// Deletes the given hash's link from links db
pub async fn delete_link(pool: &SqlitePool, hash: &str) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE from links WHERE hash = $1")
        .bind(hash)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        Err(sqlx::Error::ColumnNotFound(format!("Column {hash} not found")))
    } else {
        Ok(())
    }

}

/// Clears all links from links db
pub async fn clear_links(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE from links").execute(pool).await?;

    Ok(())
}
