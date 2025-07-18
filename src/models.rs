
use serde::{Deserialize, Serialize};


// Anthropic API Structs

#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicMessage {
    pub role: String,
    pub content: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicRequest {
    pub model: String,
    pub messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: String,
    pub content: Vec<serde_json::Value>,
    pub stop_reason: String,
    pub stop_sequence: Option<String>,
    pub model: String,
}

// OpenAI API Structs

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIMessage {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<OpenAIToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: OpenAIFunction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIResponse {
    pub id: String,
    pub choices: Vec<OpenAIChoice>,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChoice {
    pub index: u32,
    pub message: OpenAIMessage,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIStreamChoice {
    pub index: u32,
    pub delta: OpenAIMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIStreamResponse {
    pub id: String,
    pub choices: Vec<OpenAIStreamChoice>,
    pub model: String,
}
