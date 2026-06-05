mod state;
mod handlers;
mod routing_rules;

use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use state::AppState;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let litellm_url = env::var("GATEWAY_URL")
        .unwrap_or_else(|_| "http://localhost:11435/v1".to_string());

    let state = Arc::new(AppState::new(litellm_url.clone()));

    let app = Router::new()
        .route("/v1/models", get(handlers::models::list_models))
        .route("/v1/chat/completions", post(handlers::chat::intercept_chat))
        .fallback(handlers::proxy::transparent_proxy)
        .with_state(state);

    let addr = "0.0.0.0:11436";
    println!("🚀 Harness Proxy rodando em http://{}", addr);
    println!("🔗 Apontando o tráfego para o Gateway em: {}", litellm_url);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
