use serde_json::Value;

use crate::reqwest::cases::Case;

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
                "function": {
                    "name": "get_current_weather",
                    "arguments": {
                        "arguments": ["Toronto"]
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
                "Какая погода в Москве".to_string(),
            ],
        }
    }
}
