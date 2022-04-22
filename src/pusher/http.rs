use crate::PROJECT_CONFIG;
use anyhow::{anyhow, Result};
use reqwest::{Client, StatusCode};
use serde_json::Value;

pub async fn push_block_to_engine(json: &Value) -> Result<()> {
    if !PROJECT_CONFIG.push_engine {
        return Ok(());
    }

    // let json = serde_json::to_value(message)?;
    let response = Client::new()
        .post(&PROJECT_CONFIG.push_engine_url)
        .json(&json)
        .send()
        .await?;
    return if StatusCode::is_success(&response.status()) {
        Ok(())
    } else {
        Err(anyhow!(""))
    };
}
