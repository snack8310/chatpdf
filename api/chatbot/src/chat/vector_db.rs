use qdrant_client::prelude::*;
use crate::error::Result;
use crate::error::Error;

pub async fn search() -> Result<String>{
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = QdrantClient::new(Some(config)).await.map_err(|err|Error::QdrantError(err.to_string()))?;
    let collections_list = client.list_collections().await.map_err(|err|Error::QdrantError(err.to_string()))?;
    dbg!(collections_list);
    Ok("".to_owned())
}