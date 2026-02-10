use std::fs;

use serde::{Deserialize, Serialize};

use crate::BoxError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extract<T> {
    pub prompt: String,
    pub expected_data: T,
}

pub fn load<T>(file: &str) -> Result<Vec<Extract<T>>, BoxError>
where
    T: for<'de> Deserialize<'de>,
{
    let content = fs::read_to_string(file)?;
    let expected_cases = serde_json::from_str::<Vec<Extract<T>>>(&content)?;
    Ok(expected_cases)
}

pub async fn load_async<T>(file: &str) -> Result<Vec<Extract<T>>, BoxError>
where
    T: for<'de> Deserialize<'de>,
{
    let content = tokio::fs::read_to_string(file).await?;
    let expected_cases = serde_json::from_str::<Vec<Extract<T>>>(&content)?;
    Ok(expected_cases)
}
