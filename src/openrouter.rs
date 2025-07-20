use crate::config::Config;
use serde::{Deserialize, Serialize};

/// Response structure for the OpenRouter models list API
#[derive(Debug, Deserialize, Serialize)]
pub struct ModelsResponse {
    pub data: Vec<Model>,
}

/// Individual model information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub context_length: Option<i32>,
    pub architecture: Option<Architecture>,
    pub pricing: Option<Pricing>,
    pub supported_generation_methods: Option<Vec<String>>,
    pub top_provider: Option<TopProvider>,
    pub per_request_limits: Option<PerRequestLimits>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Architecture {
    pub modality: Option<String>,
    pub tokenizer: Option<String>,
    pub instruct_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pricing {
    pub prompt: Option<String>,
    pub completion: Option<String>,
    pub request: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TopProvider {
    pub context_length: Option<i32>,
    pub max_completion_tokens: Option<i32>,
    pub is_moderated: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PerRequestLimits {
    pub prompt_tokens: Option<String>,
    pub completion_tokens: Option<String>,
}

/// Fetch the list of available models from OpenRouter
pub async fn fetch_models(config: &Config) -> Result<ModelsResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/models", config.base_url);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("HTTP-Referer", "https://github.com/yourusername/ccor")
        .header("X-Title", "CCOR - Claude Connector for OpenRouter")
        .send()
        .await?;

    let models = response.json::<ModelsResponse>().await?;
    Ok(models)
}
