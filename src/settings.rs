use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    #[serde(default = "default_openrouter_base_url")]
    pub openrouter_base_url: String,
    pub openrouter_model_haiku: String,
    pub openrouter_model_sonnet: String,
    pub openrouter_model_opus: String,
}

fn default_openrouter_base_url() -> String {
    "https://openrouter.ai/api/v1".to_string()
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();
        let s = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;
        s.try_deserialize()
    }
}
