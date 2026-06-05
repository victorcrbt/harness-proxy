use axum::Json;
use serde_json::{json, Value};

pub async fn list_models() -> Json<Value> {
    let models = json!({
        "object": "list",
        "data": [
            { "id": "Auto (Local)", "object": "model" },
            { "id": "openrouter/deepseek/deepseek-v4-pro", "object": "model" },
            { "id": "openrouter/openai/o1", "object": "model" },
            { "id": "openrouter/openai/o3-mini", "object": "model" },
            { "id": "openrouter/anthropic/claude-3.7-sonnet", "object": "model" },
            { "id": "openrouter/deepseek/deepseek-v4-flash", "object": "model" },
            { "id": "openrouter/qwen/qwen3-coder-plus", "object": "model" },
            { "id": "openrouter/google/gemini-2.5-pro", "object": "model" },
            { "id": "openrouter/moonshotai/kimi-k2.5", "object": "model" },
            { "id": "openrouter/meta-llama/llama-3-70b-instruct", "object": "model" },
            { "id": "openrouter/anthropic/claude-haiku-4.5", "object": "model" },
            { "id": "openrouter/mistralai/mistral-large-2512", "object": "model" },
            { "id": "openrouter/openai/gpt-4o", "object": "model" },
            { "id": "openrouter/qwen/qwen-vl-plus", "object": "model" },
            { "id": "openrouter/google/gemini-2.5-flash", "object": "model" },
            { "id": "deepseek-r1-local", "object": "model" }
        ]
    });
    Json(models)
}
