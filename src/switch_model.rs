use crate::openrouter;
use minijinja::Environment;
use serde::{Deserialize, Serialize};

use axum::{
    extract::{Json, State},
    response::{Html, IntoResponse},
};

#[derive(Deserialize, Serialize)]
pub struct ModelSelection {
    haiku: String,
    sonnet: String,
    opus: String,
}

// Include the template at compile time
const SWITCH_MODEL_TEMPLATE: &str = include_str!("templates/switch_model.html");

/// GET /switch-model - Serve HTML interface for model selection
pub async fn switch_model_get(State(state): State<crate::AppState>) -> Html<String> {
    let cfg = state.config.read().await;
    // Fetch available models from OpenRouter
    let models_result = openrouter::fetch_models(&cfg).await;
    let models_json = match models_result {
        Ok(models) => serde_json::to_string(&models.data).unwrap_or_else(|_| "[]".to_string()),
        Err(_) => "[]".to_string(),
    };

    // Create a new minijinja environment
    let mut env = Environment::new();
    env.add_template("switch_model", SWITCH_MODEL_TEMPLATE)
        .unwrap();

    // Get the template
    let tmpl = env.get_template("switch_model").unwrap();

    // Create the context
    let ctx = minijinja::context! {
        model_haiku => cfg.model_haiku,
        model_sonnet => cfg.model_sonnet,
        model_opus => cfg.model_opus,
        models_json => models_json,
    };

    // Render the template
    let html = tmpl.render(ctx).unwrap();

    Html(html)
}

/// POST /switch-model - Save selected models to config.toml
pub async fn switch_model_post(
    State(state): State<crate::AppState>,
    Json(selection): Json<ModelSelection>,
) -> impl IntoResponse {
    let mut config = state.config.write().await;
    config.model_haiku = selection.haiku;
    config.model_opus = selection.opus;
    config.model_sonnet = selection.sonnet;
    config.write();
    "Models updated successfully"
}
