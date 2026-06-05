use serde_json::Value;

pub fn apply_routing_rules(mut payload: Value) -> Value {
    let requested_model = payload["model"].as_str().unwrap_or("Auto (Local)").to_string();
    
    if let Some(messages) = payload["messages"].as_array_mut() {
        if let Some(last_msg) = messages.last_mut() {
            
            // 1. Extração Robusta: Lida com "content" sendo String simples ou Array multimodal
            let mut text = String::new();
            
            if let Some(content_str) = last_msg["content"].as_str() {
                text = content_str.to_string();
            } else if let Some(content_array) = last_msg["content"].as_array() {
                for item in content_array {
                    if item["type"] == "text" {
                        if let Some(t) = item["text"].as_str() {
                            text.push_str(t);
                        }
                    }
                }
            }

            // 2. Aplica as Regras se houver texto identificado
            if !text.is_empty() {
                if text.starts_with('/') && text.contains(":override ") {
                    let parts: Vec<&str> = text.splitn(2, ":override ").collect();
                    if parts.len() == 2 {
                        last_msg["content"] = Value::String(parts[1].to_string());
                        return payload;
                    }
                }

                if text.starts_with('/') {
                    if let Some((tag, rest)) = text.split_once(' ') {
                        let target_model = match tag {
                            "/arch"   => Some("openrouter/deepseek/deepseek-v4-pro"),
                            "/math"   => Some("openrouter/openai/o1"),
                            "/agile"  => Some("openrouter/openai/o3-mini"),
                            "/spec"   => Some("openrouter/anthropic/claude-sonnet-4.5"),
                            "/code"   => Some("openrouter/qwen/qwen3-coder-plus"),
                            "/doc"    => Some("openrouter/google/gemini-2.5-pro"),
                            "/legacy" => Some("openrouter/moonshotai/kimi-k2.5"),
                            "/review" => Some("openrouter/meta-llama/llama-3-70b-instruct"),
                            "/log"    => Some("openrouter/anthropic/claude-haiku-4.5"),
                            "/strict" => Some("openrouter/mistralai/mistral-large-2512"),
                            "/text"   => Some("openrouter/openai/gpt-4o"),
                            "/vision" => Some("openrouter/qwen/qwen-vl-plus"),
                            "/fast"   => Some("openrouter/google/gemini-2.5-flash"),
                            "/local"  => Some("deepseek-r1-local"),
                            _ => None,
                        };

                        if let Some(model) = target_model {
                            last_msg["content"] = Value::String(rest.to_string());
                            payload["model"] = Value::String(model.to_string());
                            return payload;
                        }
                    }
                }

                // 3. Aplicação do Roteamento Inteligente
                if requested_model == "Auto (Local)" {
                    if text.contains("<|fim_prefix|>") || text.contains("<|file_separator|>") {
                        payload["model"] = Value::String("openrouter/deepseek/deepseek-v4-flash".to_string());
                    } else if text.len() > 80_000 {
                        payload["model"] = Value::String("openrouter/google/gemini-2.5-pro".to_string());
                    } else {
                        payload["model"] = Value::String("openrouter/openai/gpt-4o".to_string());
                    }
                }
            }
        }
    }
    
    // REDE DE SEGURANÇA: Impede que o proxy envie o nome falso para o Gateway caso a leitura do texto falhe
    if payload["model"].as_str().unwrap_or("") == "Auto (Local)" {
        payload["model"] = Value::String("openrouter/openai/gpt-4o".to_string());
    }

    payload
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_auto_local_fim_routing() {
        let payload = json!({ "model": "Auto (Local)", "messages": [{ "content": "código <|fim_prefix|> mais código" }] });
        let result = apply_routing_rules(payload);
        assert_eq!(result["model"], "openrouter/deepseek/deepseek-v4-flash");
    }

    #[test]
    fn test_explicit_command_routing() {
        let payload = json!({ "model": "Auto (Local)", "messages": [{ "content": "/spec crie a arquitetura" }] });
        let result = apply_routing_rules(payload);
        assert_eq!(result["model"], "openrouter/anthropic/claude-3.7-sonnet");
        assert_eq!(result["messages"][0]["content"], "crie a arquitetura");
    }

    #[test]
    fn test_override_command() {
        let payload = json!({ "model": "openrouter/google/gemini-2.5-flash", "messages": [{ "content": "/spec:override crie a arquitetura" }] });
        let result = apply_routing_rules(payload);
        assert_eq!(result["model"], "openrouter/google/gemini-2.5-flash");
        assert_eq!(result["messages"][0]["content"], "crie a arquitetura");
    }
}
