use std::collections::HashMap;

use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("db request failed")]
    RequestError(String),

    #[error("invalid data returned")]
    RetrievalError(String),

    #[error("duplicate data exists")]
    DuplicationError(),
}

#[derive(Deserialize, Serialize)]
pub struct Shortcut {
    pub link: String,
    pub hash: String,
}

pub async fn init_db_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    tracing::debug!("Initialized db");
    Client::new(&config)
}

/// Add link with generated hash to db
pub async fn add_shortcut(
    client: &Client,
    table_name: &str,
    shortcut: &Shortcut,
) -> Result<(), DbError> {
    let link_av = AttributeValue::S(shortcut.link.to_string());
    let hash_av = AttributeValue::S(shortcut.hash.to_string());

    if get_shortcut(client, table_name, &shortcut.hash)
        .await
        .is_ok()
    {
        return Err(DbError::DuplicationError());
    }

    let request = client
        .put_item()
        .table_name(table_name)
        .item("link", link_av)
        .item("link_hash", hash_av);

    tracing::debug!("Executing request [{request:?}] to add shortcut to db");

    request
        .send()
        .await
        .map_err(|e| DbError::RequestError(e.to_string()))?;

    tracing::debug!(
        "Added link {} with hash {} to db",
        shortcut.link,
        shortcut.hash
    );

    Ok(())
}

/// Get the given hash's link from db
pub async fn get_shortcut(
    client: &Client,
    table_name: &str,
    hash: &str,
) -> Result<String, DbError> {
    let request = client
        .query()
        .table_name(table_name)
        .key_condition_expression("link_hash = :hash")
        .expression_attribute_values(":hash", AttributeValue::S(hash.to_string()))
        .projection_expression("link");

    tracing::debug!("Executing request [{request:?}] to get shortcut from db using hash");

    let response = request
        .send()
        .await
        .map_err(|e| DbError::RequestError(e.to_string()))?;

    match response.items {
        None => Err(DbError::RetrievalError(
            "Query response did not have any items to check".to_string(),
        )),
        Some(items) => match items.len() {
            1 => {
                let link = items[0].get("link");
                if link.is_none() || link.unwrap().as_s().is_err() {
                    Err(DbError::RetrievalError(
                        "Query response item did not provide a valid link".to_string(),
                    ))
                } else {
                    let link_str = link.unwrap().as_s().unwrap().to_string();
                    tracing::debug!("Fetched link {link_str} from hash {hash}");
                    Ok(link_str)
                }
            }
            l => Err(DbError::RetrievalError(format!(
                "Query had {l} entries instead of one"
            ))),
        },
    }
}

pub async fn get_all_shortcuts(
    client: &Client,
    table_name: &str,
) -> Result<Vec<HashMap<String, AttributeValue>>, DbError> {
    let request = client.scan().table_name(table_name);

    tracing::debug!("Executing request [{request:?}] to get shortcut from db using hash");

    let response = request
        .send()
        .await
        .map_err(|e| DbError::RequestError(e.to_string()))?;

    match response.items {
        None => Err(DbError::RetrievalError(
            "Scan response did not function properly".to_string(),
        )),
        Some(items) => Ok(items),
    }
}

/// Deletes the given hash's shortcut from db
pub async fn delete_shortcut(client: &Client, table_name: &str, hash: &str) -> Result<(), DbError> {
    let request = client
        .delete_item()
        .table_name(table_name)
        .key("link_hash", AttributeValue::S(hash.to_string()));

    tracing::debug!("Executing request [{request:?}] to get shortcut from db using hash");

    let _response = request
        .send()
        .await
        .map_err(|e| DbError::RequestError(e.to_string()))?;

    tracing::debug!("Deleted link with {hash}");
    Ok(())
}


#[cfg(test)]
#[tokio::test]
async fn test_init_db() -> Result<(), aws_sdk_dynamodb::Error> {
    dotenv::dotenv().ok();
    let client = init_db_client().await;
    let resp = client.list_tables().send().await?;

    assert_eq!(resp.table_names().len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_add_to_db() -> Result<(), DbError> {
    dotenv::dotenv().ok();
    let client = init_db_client().await;
    let table_name = std::env::var("AWS_TABLE_NAME").unwrap();

    let shortcut = Shortcut {
        link: "https://www.google.com".to_string(),
        hash: "hello-world".to_string(),
    };
    add_shortcut(&client, &table_name, &shortcut).await?;

    let link = get_shortcut(&client, &table_name, &shortcut.hash)
        .await
        .unwrap();

    assert_eq!(link, shortcut.link);

    delete_shortcut(&client, &table_name, &shortcut.hash).await?;

    match get_shortcut(&client, &table_name, &shortcut.hash).await {
        Ok(_) => Err(DbError::DuplicationError()),
        Err(_) => Ok(()),
    }
}
