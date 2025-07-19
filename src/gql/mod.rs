pub mod client;

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::{AppError, AppResult};

/// Generic GraphQL POST function to LeetCode
pub async fn leetcode_gql<T: DeserializeOwned>(
    query: &str,
    variables: Value,
) -> AppResult<T> {
    let body = serde_json::json!({
        "query": query,
        "variables": variables,
    });

    let res = Client::new()
        .post("https://leetcode.com/graphql")
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::InternalWith(e.to_string()))?;

    let parsed: T = res
        .json()
        .await
        .map_err(|e| AppError::InternalWith(e.to_string()))?;

    Ok(parsed)
}
