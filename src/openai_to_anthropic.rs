use crate::models::*;
use serde_json::json;

pub fn format_openai_to_anthropic(resp: OpenAIResponse) -> AnthropicResponse {
    let choice = &resp.choices[0];
    let mut content = Vec::new();

    if let Some(text) = &choice.message.content {
        content.push(json!({ "type": "text", "text": text }));
    }

    if let Some(tool_calls) = &choice.message.tool_calls {
        for tool_call in tool_calls {
            content.push(json!({
                "type": "tool_use",
                "id": tool_call.id,
                "name": tool_call.function.name,
                "input": serde_json::from_str::<serde_json::Value>(&tool_call.function.arguments).unwrap_or(json!({})),
            }));
        }
    }

    AnthropicResponse {
        id: resp.id,
        response_type: "message".to_string(),
        role: "assistant".to_string(),
        content,
        stop_reason: if choice.finish_reason == "tool_calls" {
            "tool_use".to_string()
        } else {
            "end_turn".to_string()
        },
        stop_sequence: None,
        model: resp.model,
    }
}
