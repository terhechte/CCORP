mod anthropic_to_openai;
mod models;
mod openai_to_anthropic;
mod settings;

use axum::{
    Router,
    body::Body,
    extract::{Json, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::post,
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
                .unwrap_or_else(|_| "ccor=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut logging_path: Option<String> = None;
    let args: Vec<String> = std::env::args().collect();
    if let Some(index) = args.iter().position(|arg| arg == "--logging") {
        if let Some(path) = args.get(index + 1) {
            logging_path = Some(path.clone());
            std::fs::create_dir_all(path).expect("Failed to create logging directory");
            println!("Logging requests and responses to: {path}");
        } else {
            eprintln!("--logging flag requires a path argument.");
            std::process::exit(1);
        }
    }

    let settings = Settings::new().expect("Failed to load settings");

    println!("Using the following model mappings:");
    println!("- Haiku: {}", settings.openrouter_model_haiku);
    println!("- Sonnet: {}", settings.openrouter_model_sonnet);
    println!("- Opus: {}", settings.openrouter_model_opus);

    let shared_settings = Arc::new(settings);
    let shared_logging_path = Arc::new(logging_path);

    let app = Router::new()
        .route("/v1/messages", post(messages_handler))
        .with_state((shared_settings, shared_logging_path));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3073").await.unwrap();
    println!("Listening on http://0.0.0.0:3073");
    axum::serve(listener, app).await.unwrap();
}

async fn messages_handler(
    State((settings, logging_path)): State<(Arc<Settings>, Arc<Option<String>>)>,
    headers: HeaderMap,
    Json(payload): Json<AnthropicRequest>,
) -> impl IntoResponse {
    let openai_request = anthropic_to_openai::format_anthropic_to_openai(payload, &settings);

    if let Some(path) = logging_path.as_ref() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let request_path = format!("{path}/{timestamp}-request.json");
        let request_json = serde_json::to_string_pretty(&openai_request).unwrap();
        std::fs::write(request_path, request_json).expect("Failed to write request log");
    }
    let client = Client::new();
    let api_key = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .expect("Please set the ANTHROPIC_AUTH_TOKEN to your OpenRouter Key")
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

            let mut full_response = String::new();
            while let Some(item) = stream.next().await {
                let chunk = item.unwrap();
                full_response.push_str(&String::from_utf8_lossy(&chunk));
                let chunk_str = String::from_utf8_lossy(&chunk);
                for line in chunk_str.split("

") {
                    if let Some(data) = line.strip_prefix("data: ") {
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
data: {anthropic_stream_event}

");
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
data: {message_stop}

");
            yield Ok::<_, axum::Error>(sse_event.into_bytes());

            if let Some(path) = logging_path.as_ref() {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                let response_path = format!("{path}/{timestamp}-response.json");
                std::fs::write(response_path, full_response).expect("Failed to write response log");
            }
        };

        let body = Body::from_stream(stream);

        Response::builder()
            .header(header::CONTENT_TYPE, "text/event-stream")
            .body(body)
            .unwrap()
    } else {
        let res = client
            .post(format!("{}/chat/completions", settings.openrouter_base_url))
            .bearer_auth(api_key)
            .json(&openai_request)
            .send()
            .await
            .unwrap();

        if !res.status().is_success() {
            return (res.status(), res.text().await.unwrap_or_default()).into_response();
        }

        let openai_response: models::OpenAIResponse = res.json().await.unwrap();
        let anthropic_response =
            openai_to_anthropic::format_openai_to_anthropic(openai_response.clone());

        if let Some(path) = logging_path.as_ref() {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let response_path = format!("{path}/{timestamp}-response.json");
            let response_json = serde_json::to_string_pretty(&anthropic_response).unwrap();
            std::fs::write(response_path, response_json).expect("Failed to write response log");
        }

        (StatusCode::OK, Json(anthropic_response)).into_response()
    }
}
