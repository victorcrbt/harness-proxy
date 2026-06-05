use axum::{
    body::Body,
    extract::{Request, State},
    response::IntoResponse,
};
use serde_json::Value;
use std::sync::Arc;
use crate::state::AppState;

pub async fn transparent_proxy(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
) -> impl IntoResponse {
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");
    
    let target_url = if query.is_empty() {
        format!("{}{}", state.litellm_url, path.replace("/v1/v1", "/v1"))
    } else {
        format!("{}{}?{}", state.litellm_url, path, query)
    };

    let method = req.method().clone();
    let headers = req.headers().clone();
    
    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX).await.unwrap();

    // Tenta interpretar o payload como JSON para ler a flag de stream. 
    // Se não for JSON (ex: upload de áudio), assume false como segurança.
    let is_stream = serde_json::from_slice::<Value>(&body_bytes)
        .map(|json| json["stream"].as_bool().unwrap_or(false))
        .unwrap_or(false);

    let response = state.client.request(method, target_url)
        .headers(headers)
        .body(body_bytes)
        .send()
        .await
        .expect("Falha no repasse do proxy transparente");

    let mut resp_builder = axum::response::Response::builder()
        .status(response.status());
    
    for (key, value) in response.headers() {
        resp_builder = resp_builder.header(key, value);
    }

    if is_stream {
        let stream = response.bytes_stream();
        resp_builder.body(Body::from_stream(stream)).unwrap()
    } else {
        let bytes = response.bytes().await.unwrap();
        resp_builder.body(Body::from(bytes)).unwrap()
    }
}
