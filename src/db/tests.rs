use aws_sdk_dynamodb::Error;
use dotenv::dotenv;

use super::init_db_client;

#[tokio::test]
async fn test_init_db() -> Result<(), Error> {
    // dotenv().ok();
    let client = init_db_client().await;
    let resp = client.list_tables().send().await?;

    assert_eq!(resp.table_names().len(), 1);

    Ok(())
}
