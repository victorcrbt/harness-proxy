use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, header},
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use std::sync::Arc;
use crate::{state::AppState, routing_rules::apply_routing_rules};

pub async fn intercept_chat(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let is_stream = payload["stream"].as_bool().unwrap_or(false);

    let modified_payload = apply_routing_rules(payload);
    let litellm_endpoint = format!("{}/chat/completions", state.litellm_url);
    
    // Remove cabeçalhos da origem que podem causar rejeição no gateway
    let mut clean_headers = headers.clone();
    clean_headers.remove(header::HOST);
    clean_headers.remove(header::CONTENT_LENGTH);

    let response = state.client.post(&litellm_endpoint)
        .headers(clean_headers)
        .json(&modified_payload)
        .send()
        .await
        .expect("Falha ao comunicar com o Gateway");

    let mut resp_builder = axum::response::Response::builder()
        .status(response.status());
    
    // Filtra cabeçalhos da resposta para evitar dupla compressão ou dessincronia na IDE
    for (key, value) in response.headers() {
        if key != header::TRANSFER_ENCODING 
            && key != header::CONTENT_ENCODING 
            && key != header::CONNECTION 
            && key != header::CONTENT_LENGTH {
            resp_builder = resp_builder.header(key, value);
        }
    }

    if is_stream {
        let stream = response.bytes_stream();
        resp_builder.body(Body::from_stream(stream)).unwrap()
    } else {
        let bytes = response.bytes().await.unwrap();
        resp_builder.body(Body::from(bytes)).unwrap()
    }
}
