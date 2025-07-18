
use crate::models::*;
use crate::settings::Settings;
use serde_json::json;

pub fn map_model(anthropic_model: &str, settings: &Settings) -> String {
    if anthropic_model.contains("haiku") {
        settings.openrouter_model_haiku.clone()
    } else if anthropic_model.contains("sonnet") {
        settings.openrouter_model_sonnet.clone()
    } else if anthropic_model.contains("opus") {
        settings.openrouter_model_opus.clone()
    } else if anthropic_model.contains('/') {
        anthropic_model.to_string()
    } else {
        anthropic_model.to_string()
    }
}

pub fn format_anthropic_to_openai(req: AnthropicRequest, settings: &Settings) -> OpenAIRequest {
    let mut openapi_messages = Vec::new();

    if let Some(system) = req.system {
        if let Some(system_str) = system.as_str() {
            openapi_messages.push(OpenAIMessage {
                role: "system".to_string(),
                content: Some(system_str.to_string()),
                tool_calls: None,
                tool_call_id: None,
            });
        }
    }

    for message in req.messages {
        match message.role.as_str() {
            "user" => {
                if let Some(content_array) = message.content.as_array() {
                    let mut user_text = String::new();
                    for content in content_array {
                        if content["type"] == "text" {
                            user_text.push_str(content["text"].as_str().unwrap_or(""));
                        } else if content["type"] == "tool_result" {
                            openapi_messages.push(OpenAIMessage {
                                role: "tool".to_string(),
                                content: Some(content["content"].to_string()),
                                tool_call_id: Some(content["tool_use_id"].as_str().unwrap_or("").to_string()),
                                tool_calls: None,
                            });
                        }
                    }
                    if !user_text.is_empty() {
                        openapi_messages.push(OpenAIMessage {
                            role: "user".to_string(),
                            content: Some(user_text),
                            tool_calls: None,
                            tool_call_id: None,
                        });
                    }
                } else if let Some(content_str) = message.content.as_str() {
                    openapi_messages.push(OpenAIMessage {
                        role: "user".to_string(),
                        content: Some(content_str.to_string()),
                        tool_calls: None,
                        tool_call_id: None,
                    });
                }
            }
            "assistant" => {
                let mut assistant_message = OpenAIMessage {
                    role: "assistant".to_string(),
                    content: None,
                    tool_calls: None,
                    tool_call_id: None,
                };
                let mut tool_calls = Vec::new();
                if let Some(content_array) = message.content.as_array() {
                    let mut assistant_text = String::new();
                    for content in content_array {
                        if content["type"] == "text" {
                            assistant_text.push_str(content["text"].as_str().unwrap_or(""));
                        } else if content["type"] == "tool_use" {
                            tool_calls.push(OpenAIToolCall {
                                id: content["id"].as_str().unwrap_or("").to_string(),
                                tool_type: "function".to_string(),
                                function: OpenAIFunction {
                                    name: content["name"].as_str().unwrap_or("").to_string(),
                                    arguments: content["input"].to_string(),
                                },
                            });
                        }
                    }
                    if !assistant_text.is_empty() {
                        assistant_message.content = Some(assistant_text);
                    }
                }
                if !tool_calls.is_empty() {
                    assistant_message.tool_calls = Some(tool_calls);
                }
                openapi_messages.push(assistant_message);
            }
            _ => {}
        }
    }

    let mut tools = None;
    if let Some(anthropic_tools) = req.tools {
        tools = Some(
            anthropic_tools
                .into_iter()
                .map(|t| {
                    json!({
                        "type": "function",
                        "function": {
                            "name": t["name"], 
                            "description": t["description"], 
                            "parameters": t["input_schema"], 
                        }
                    })
                })
                .collect(),
        );
    }

    OpenAIRequest {
        model: map_model(&req.model, settings),
        messages: openapi_messages,
        temperature: req.temperature,
        stream: req.stream,
        tools,
    }
}
