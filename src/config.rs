use dotenvy::dotenv;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs;

/// TOML configuration structure
#[derive(Deserialize, Serialize)]
struct JsonConfig {
    port: u16,
    models: ModelConfig,
}

#[derive(Deserialize, Serialize)]
struct ModelConfig {
    haiku: String,
    sonnet: String,
    opus: String,
}

/// Runtime configuration loaded from environment variables.
#[derive(Clone, Debug)]
pub struct Config {
    /// The port to listen on
    pub port: u16,
    /// Base URL for the OpenRouter API (e.g., https://openrouter.ai/api/v1)
    pub base_url: String,
    /// API key for authenticating with OpenRouter
    pub api_key: String,
    /// Override model name for Claude 3.5 Haiku
    pub model_haiku: String,
    /// Override model name for Claude Sonnet 4
    pub model_sonnet: String,
    /// Override model name for Claude Opus 4
    pub model_opus: String,
}

impl Config {
    /// Load configuration from `config.json` and `.env` file.
    pub fn from_env() -> Self {
        // Load environment variables from .env file
        dotenv().ok();

        // Load API key from environment (must be present)
        let api_key = env::var("OPENROUTER_API_KEY")
            .expect("Environment variable OPENROUTER_API_KEY must be set");

        // Load config.json
        let config: JsonConfig = serde_json::from_str(
            &fs::read_to_string("config.json").expect("Could not read config.json file"),
        )
        .expect("Could not read config.json file");

        Config {
            port: config.port,
            base_url: default_openrouter_base_url(),
            api_key,
            model_haiku: config.models.haiku,
            model_sonnet: config.models.sonnet,
            model_opus: config.models.opus,
        }
    }

    /// Write configuration to `config.json` (excluding secrets like api_key).
    pub fn write(&self) {
        let config_out = JsonConfig {
            port: self.port,
            models: ModelConfig {
                haiku: self.model_haiku.clone(),
                sonnet: self.model_sonnet.clone(),
                opus: self.model_opus.clone(),
            },
        };

        let json_string =
            serde_json::to_string_pretty(&config_out).expect("Failed to serialize configuration");

        fs::write("config.json", json_string).expect("Failed to write config.json");
    }
}

fn default_openrouter_base_url() -> String {
    "https://openrouter.ai/api/v1".to_string()
}
