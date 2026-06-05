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
