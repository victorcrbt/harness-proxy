use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::fs;
use dotenvy::dotenv;

struct TestCase {
    name: &'static str,
    payload: Value,
    expected_model: &'static str,
}

struct TestResult {
    id: usize,
    name: String,
    expected: String,
    actual: String,
    passed: bool,
    details: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let proxy_url = "http://127.0.0.1:11436/v1/chat/completions";
    
    // Carrega a chave do .env
    let api_key = env::var("LITELLM_MASTER_KEY").expect("LITELLM_MASTER_KEY não encontrada no .env");

    // Gera um texto com mais de 80.000 caracteres para testar o roteamento por volume
    let massive_text = "A".repeat(80_001);

    let test_cases = vec![
        TestCase {
            name: "1. Fallback do Auto (Local)",
            payload: json!({ "model": "Auto (Local)", "messages": [{"role": "user", "content": [{"type": "text", "text": "Oi"}]}], "stream": false }),
            expected_model: "gpt-4o",
        },
        TestCase {
            name: "2. Detecção de FIM (Autocomplete)",
            payload: json!({ "model": "Auto (Local)", "messages": [{"role": "user", "content": [{"type": "text", "text": "<|fim_prefix|>"}]}], "stream": false }),
            expected_model: "deepseek-v4-flash",
        },
        TestCase {
            name: "3. Roteamento por Tag (/spec)",
            payload: json!({ "model": "Auto (Local)", "messages": [{"role": "user", "content": [{"type": "text", "text": "/spec docs"}]}], "stream": false }),
            expected_model: "claude-sonnet-4.5",
        },
        TestCase {
            name: "4. Roteamento por Tag (/code)",
            payload: json!({ "model": "Auto (Local)", "messages": [{"role": "user", "content": [{"type": "text", "text": "/code test"}]}], "stream": false }),
            expected_model: "qwen3-coder-plus",
        },
        TestCase {
            name: "5. Roteamento por Tag (/arch)",
            payload: json!({ "model": "Auto (Local)", "messages": [{"role": "user", "content": [{"type": "text", "text": "/arch sys"}]}], "stream": false }),
            expected_model: "deepseek-v4-pro",
        },
        TestCase {
            name: "6. Roteamento Local Seguro (/local)",
            payload: json!({ "model": "Auto (Local)", "messages": [{"role": "user", "content": [{"type": "text", "text": "/local token"}]}], "stream": false }),
            expected_model: "deepseek-r1-local",
        },
        TestCase {
            name: "7. Carga de Contexto Massiva (>80k chars)",
            payload: json!({ "model": "Auto (Local)", "messages": [{"role": "user", "content": [{"type": "text", "text": massive_text}]}], "stream": false }),
            expected_model: "gemini-2.5-pro",
        },
        TestCase {
            name: "8. Comando Override Estrito",
            payload: json!({ "model": "openrouter/openai/o1", "messages": [{"role": "user", "content": [{"type": "text", "text": "/spec:override force"}]}], "stream": false }),
            expected_model: "openrouter/openai/o1",
        },
    ];

    let client = Client::new();
    let mut results: Vec<TestResult> = Vec::new();
    let mut total_passed = 0;

    println!("Iniciando bateria estendida de testes...\n");

    for (idx, test) in test_cases.iter().enumerate() {
        let payload_debug = serde_json::to_string_pretty(&test.payload).unwrap_or_default();
        
        let res = client.post(proxy_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&test.payload)
            .send()
            .await;

        let mut actual_model = String::new();
        let mut passed = false;
        let mut details = String::new();

        match res {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    let body: Value = response.json().await?;
                    if let Some(model_str) = body["model"].as_str() {
                        actual_model = model_str.to_string();
                        passed = actual_model.contains(test.expected_model);
                        if !passed {
                            details = format!("Modelo retornado difere do esperado.\nPayload Enviado:\n```json\n{}\n```", payload_debug);
                        }
                    } else {
                        actual_model = "Erro: Sem campo 'model'".to_string();
                        details = format!("Resposta inválida:\n```json\n{}\n```", body);
                    }
                } else {
                    actual_model = format!("Erro HTTP {}", status);
                    let error_body = response.text().await.unwrap_or_else(|_| "Falha ao ler corpo do erro".to_string());
                    details = format!("O provedor ou proxy retornou erro.\nCorpo da Resposta:\n```json\n{}\n```", error_body);
                }
            }
            Err(e) => {
                actual_model = "Erro de Rede".to_string();
                details = format!("Falha de conexão: {}", e);
            }
        }

        if passed { total_passed += 1; }
        
        println!("[{}] {} | Retornado: {}", if passed { "✅" } else { "❌" }, test.name, actual_model);
        
        results.push(TestResult {
            id: idx + 1,
            name: test.name.to_string(),
            expected: test.expected_model.to_string(),
            actual: actual_model,
            passed,
            details,
        });
    }

    let mut md_content = String::from("# Relatório de Testes: Harness Proxy (Estendido)\n\n");
    md_content.push_str("| ID | Cenário | Modelo Esperado (Alvo) | Modelo Retornado (Real) | Status |\n|---|---|---|---|---|\n");
    
    let mut failures_md = String::from("\n## Anexo de Diagnóstico (Falhas)\n\n");
    let mut has_failures = false;

    for r in results {
        let status_icon = if r.passed { "✅ Passou" } else { "❌ Falhou" };
        md_content.push_str(&format!("| {} | {} | `{}` | `{}` | {} |\n", r.id, r.name, r.expected, r.actual, status_icon));
        
        if !r.passed {
            has_failures = true;
            failures_md.push_str(&format!("### Falha no Teste {}: {}\n**Esperado:** `{}` | **Retornado:** `{}`\n{}\n\n---\n", r.id, r.name, r.expected, r.actual, r.details));
        }
    }

    md_content.push_str(&format!("\n**Resultado Final:** {}/{} testes passaram.\n", total_passed, test_cases.len()));
    
    if has_failures {
        md_content.push_str(&failures_md);
    }

    fs::write("test_results.md", md_content)?;
    println!("\nRelatório detalhado gerado: test_results.md");

    Ok(())
}