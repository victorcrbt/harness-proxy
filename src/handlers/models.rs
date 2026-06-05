use axum::Json;
use serde_json::{json, Value};
use std::fs;

pub async fn list_models() -> Json<Value> {
    let mut data = vec![json!({
        "id": "Auto (Local)",
        "object": "model",
        "created": 1686935002,
        "owned_by": "harness"
    })];

    if let Ok(content) = fs::read_to_string("models.json") {
        if let Ok(models) = serde_json::from_str::<Vec<String>>(&content) {
            for model_id in models {
                data.push(json!({
                    "id": model_id,
                    "object": "model",
                    "created": 1686935002,
                    "owned_by": "harness-config"
                }));
            }
        } else {
            eprintln!("Aviso: Falha de parsing no models.json. O arquivo não contém um array de strings válido.");
        }
    } else {
        eprintln!("Aviso: Arquivo models.json não encontrado no diretório de execução. Fallback ativo.");
    }

    Json(json!({
        "object": "list",
        "data": data
    }))
}

// Rota para ler o arquivo (retorna [] se não existir)
pub async fn get_models_config() -> Json<Value> {
    let content = fs::read_to_string("models.json").unwrap_or_else(|_| "[]".to_string());
    let models: Value = serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!([]));
    Json(models)
}

// Rota para sobrescrever o arquivo
pub async fn update_models_config(Json(payload): Json<Value>) -> Json<Value> {
    if let Err(e) = fs::write("models.json", payload.to_string()) {
        eprintln!("Erro ao salvar models.json: {}", e);
    }
    Json(serde_json::json!({ "status": "success" }))
}
