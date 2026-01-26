use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

pub struct Client {
    host: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct LlmRequest {
    model: String,
    messages: Vec<LlmMessage>,
    stream: bool,
    think: bool,
    tools: Option<Vec<Value>>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct LlmMessage {
    pub role: String,
    pub content: String,
    pub tool_name: Option<String>,
    pub tool_calls: Option<Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LlmResponseChunk {
    pub message: LlmMessage,
    pub done: bool,
}

type BoxError = Box<dyn Error + Send + Sync>;

impl Client {
    pub fn new(host: &str) -> Self {
        Client {
            host: host.to_string(),
        }
    }

    pub fn generate(
        &self,
        prompt: &str,
        model: &str,
        tools: Vec<Value>,
    ) -> Result<Vec<LlmResponseChunk>, BoxError> {
        let url = format!("{}/api/chat", self.host);
        let query = LlmRequest {
            model: model.to_string(),
            messages: vec![LlmMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
                tool_name: None,
                tool_calls: None,
            }],
            stream: false,
            think: false,
            tools: Some(tools),
        };
        let body = serde_json::to_string(&query)?;

        let response = reqwest::blocking::Client::new().post(url).body(body).send();

        let respres = match response {
            Ok(response) => response,
            Err(err) => return Err(Box::new(err)),
        };
        let resp = respres.text()?;

        let vresp = resp
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|chunk| serde_json::from_str(chunk))
            .collect::<Result<Vec<LlmResponseChunk>, _>>()?;
        Ok(vresp)
    }
}
