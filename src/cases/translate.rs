use std::fs;

use serde::{Deserialize, Serialize};

use crate::BoxError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Translate<T> {
    pub source: String,
    pub meta: T,
}

pub fn load<T>(file: &str) -> Result<Vec<Translate<T>>, BoxError>
where
    T: for<'de> Deserialize<'de>,
{
    let content = fs::read_to_string(file)?;
    let expected_cases = serde_json::from_str::<Vec<Translate<T>>>(&content)?;
    Ok(expected_cases)
}

pub async fn load_async<T>(file: &str) -> Result<Vec<Translate<T>>, BoxError>
where
    T: for<'de> Deserialize<'de>,
{
    let content = tokio::fs::read_to_string(file).await?;
    let expected_cases = serde_json::from_str::<Vec<Translate<T>>>(&content)?;
    Ok(expected_cases)
}
