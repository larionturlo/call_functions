use serde_json::Value;

pub struct Case {
    pub prompt: String,
    pub tools: Vec<Value>,
    pub expected_tool_call: Value,
}

pub mod extract;
pub mod simple;
