mod anthropic_to_openai;
mod models;
mod openai_to_anthropic;
mod settings;

use axum::{
    body::Body,
    extract::{Json, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use futures_util::stream::StreamExt;
use models::{AnthropicRequest, OpenAIStreamResponse};
use reqwest::Client;
use serde_json::json;
use settings::Settings;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_sse=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let settings = Settings::new().expect("Failed to load settings");

    println!("Using the following model mappings:");
    println!("- Haiku: {}", settings.openrouter_model_haiku);
    println!("- Sonnet: {}", settings.openrouter_model_sonnet);
    println!("- Opus: {}", settings.openrouter_model_opus);

    let shared_settings = Arc::new(settings);

    let app = Router::new()
        .route("/v1/messages", post(messages_handler))
        .with_state(shared_settings);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn messages_handler(
    State(settings): State<Arc<Settings>>,
    headers: HeaderMap,
    Json(payload): Json<AnthropicRequest>,
) -> impl IntoResponse {
    let openai_request = anthropic_to_openai::format_anthropic_to_openai(payload, &settings);
    let client = Client::new();
    let api_key = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or(&settings.openrouter_api_key)
        .to_string();

    if openai_request.stream.unwrap_or(false) {
        let settings = settings.clone();
        let stream = async_stream::stream! {
            let res = client
                .post(format!(
                    "{}/chat/completions",
                    settings.openrouter_base_url
                ))
                .bearer_auth(api_key)
                .json(&openai_request)
                .send()
                .await
                .unwrap();

            if !res.status().is_success() {
                // This part is tricky because we are in a stream.
                // We can't easily return a different response.
                // For now, we'll just log the error and end the stream.
                tracing::error!("OpenRouter request failed: {}", res.text().await.unwrap_or_default());
                return;
            }

            let mut stream = res.bytes_stream();

            while let Some(item) = stream.next().await {
                let chunk = item.unwrap();
                let chunk_str = String::from_utf8_lossy(&chunk);
                for line in chunk_str.split("

") {
                    if line.starts_with("data: ") {
                        let data = &line[6..];
                        if data == "[DONE]" {
                            break;
                        }
                        if let Ok(stream_res) = serde_json::from_str::<OpenAIStreamResponse>(data) {
                            let choice = &stream_res.choices[0];
                            if let Some(content) = &choice.delta.content {
                                let anthropic_stream_event = json!({
                                    "type": "content_block_delta",
                                    "index": 0,
                                    "delta": {
                                        "type": "text_delta",
                                        "text": content
                                    }
                                });
                                let sse_event = format!("event: content_block_delta
data: {}

", anthropic_stream_event);
                                yield Ok::<_, axum::Error>(sse_event.into_bytes());
                            }
                        }
                    }
                }
            }

            let message_stop = json!({
                "type": "message_stop"
            });
            let sse_event = format!("event: message_stop
data: {}

", message_stop);
            yield Ok::<_, axum::Error>(sse_event.into_bytes());
        };

        let body = Body::from_stream(stream);

        Response::builder()
            .header(header::CONTENT_TYPE, "text/event-stream")
            .body(body)
            .unwrap()
    } else {
        let res = client
            .post(format!(
                "{}/chat/completions",
                settings.openrouter_base_url
            ))
            .bearer_auth(api_key)
            .json(&openai_request)
            .send()
            .await
            .unwrap();

        if !res.status().is_success() {
            return (res.status(), res.text().await.unwrap_or_default()).into_response();
        }

        let openai_response = res.json().await.unwrap();
        let anthropic_response = openai_to_anthropic::format_openai_to_anthropic(openai_response);
        (StatusCode::OK, Json(anthropic_response)).into_response()
    }
}