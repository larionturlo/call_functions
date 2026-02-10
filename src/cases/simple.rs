use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{BoxError, cases::Case};

#[derive(Serialize, Deserialize)]
pub struct Simple {
    tools: Vec<Value>,
    tool_call: Value,
    prompts: Vec<String>,
}

impl Simple {
    pub fn new(tools: Vec<Value>, tool_call: Value, prompts: Vec<String>) -> Self {
        Simple {
            tools,
            tool_call,
            prompts,
        }
    }

    pub fn try_to_file(&self, file_path: &str) -> Result<(), BoxError> {
        let json = serde_json::to_string(self)?;
        fs::write(file_path, json)?;
        Ok(())
    }

    pub fn try_from_file(file_path: &str) -> Result<Self, BoxError> {
        let json = fs::read_to_string(file_path)?;
        let result = serde_json::from_str(&json)?;
        Ok(result)
    }

    pub fn from_file_or_default(file_path: &str) -> Self {
        Self::try_from_file(file_path).unwrap_or_default()
    }

    pub async fn try_to_file_async(&self, file_path: &str) -> Result<(), BoxError> {
        let json = serde_json::to_string(self)?;
        tokio::fs::write(file_path, json).await?;
        Ok(())
    }

    pub async fn try_from_file_async(file_path: &str) -> Result<Self, BoxError> {
        let json = tokio::fs::read_to_string(file_path).await?;
        let result = serde_json::from_str(&json)?;
        Ok(result)
    }

    pub async fn from_file_or_default_async(file_path: &str) -> Self {
        Self::try_from_file_async(file_path)
            .await
            .unwrap_or_default()
    }
}

impl IntoIterator for Simple {
    type Item = Case;
    type IntoIter = std::vec::IntoIter<Case>;

    fn into_iter(self) -> Self::IntoIter {
        self.prompts
            .into_iter()
            .map(|prompt| Case {
                prompt: prompt.clone(),
                tools: self.tools.clone(),
                expected_tool_call: self.tool_call.clone(),
            })
            .collect::<Vec<Case>>()
            .into_iter()
    }
}

impl Default for Simple {
    fn default() -> Self {
        Simple {
            tools: vec![serde_json::json!({
              "type": "function",
              "function": {
                "name": "get_current_weather",
                "description": "Get the current weather for a location",
                "parameters": {
                  "type": "object",
                  "properties": {
                    "location": {
                      "type": "string",
                      "description": "The location to get the weather for, e.g. San Francisco, CA"
                    },
                  },
                  "required": ["location"]//, "format"]
                }
              }
            })],
            tool_call: serde_json::json!({
                "name": "get_current_weather",
                "function": {
                    "arguments": {
                        "location": "Toronto"
                    }
                }
            }),
            prompts: vec![
                "What is the weather like in Toronto?".to_string(),
                "Can you tell me the weather in Toronto?".to_string(),
                "What's the weather in Toronto like?".to_string(),
                "What's the weather like in Toronto?".to_string(),
                "weather in toronto?".to_string(),
                "weather in toronto".to_string(),
                "погода в Торонто".to_string(),
                // "Какая погода в Москве".to_string(),
            ],
        }
    }
}
