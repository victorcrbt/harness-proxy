mod state;
mod handlers;
mod routing_rules;

use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use http::{Method, header};
use reqwest::Client;
use state::AppState;
use std::{env, sync::Arc, net::SocketAddr, time::Duration, process::{Command, Stdio}};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use log::{info, error, warn};

#[cfg_attr(mobile, tauri::mobile::app_entry)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            eprintln!("🔧 Setup iniciado...");

            std::thread::spawn(|| {
                if let Err(e) = start_axum_server() {
                    error!("❌ Falha ao iniciar servidor Axum: {}", e);
                }
            });

            eprintln!("🔍 Aguardando frontend em http://127.0.0.1:3528 ...");
            let rt = tokio::runtime::Runtime::new().unwrap();
            match rt.block_on(check_frontend("http://127.0.0.1:3528", 30)) {
                Ok(()) => eprintln!("✅ Frontend pronto em http://127.0.0.1:3528"),
                Err(e) => {
                    eprintln!("❌ Frontend NÃO iniciou: {}", e);
                    eprintln!("   Verifique manualmente: cd frontend && yarn dev");
                }
            }

            info!("✅ Tauri aplicação iniciada com sucesso");
            eprintln!("✅ Aplicação iniciada com sucesso");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .unwrap_or_else(|err| {
            eprintln!("❌ Erro crítico ao executar aplicação Tauri: {}", err);
            std::process::exit(1);
        });
}

async fn start_axum_server_async() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let litellm_url = env::var("GATEWAY_URL")
        .unwrap_or_else(|_| {
            warn!("⚠️  GATEWAY_URL não definida, usando valor padrão: http://localhost:11435/v1");
            "http://localhost:11435/v1".to_string()
        });

    info!("🔗 Gateway URL configurada: {}", litellm_url);
    let state = Arc::new(AppState::new(litellm_url.clone()));

    let app = Router::new()
        .route("/v1/models", get(handlers::models::list_models))
        .route("/v1/chat/completions", post(handlers::chat::intercept_chat))
        .route(
            "/v1/config/models",
            get(handlers::models::get_models_config)
                .post(handlers::models::update_models_config),
        )
        .fallback(handlers::proxy::transparent_proxy)
        .layer(build_cors_layer())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 11436));
    
    info!("📡 Tentando vinculação ao servidor em {}", addr);
    let listener = TcpListener::bind(addr).await?;
    
    info!("🚀 Servidor Axum rodando em {}", addr);
    info!("✅ Proxy ativo e aguardando requisições em http://127.0.0.1:11436");
    
    axum::serve(listener, app).await?;
    Ok(())
}

fn start_axum_server() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(start_axum_server_async())
}

fn build_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            "http://localhost:3528".parse().unwrap(),
            "http://127.0.0.1:3528".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
}

async fn check_frontend(url: &str, max_retries: u32) -> Result<(), String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .map_err(|e| format!("erro ao criar cliente HTTP: {}", e))?;

    for i in 1..=max_retries {
        match client.get(url).send().await {
            Ok(resp) => {
                eprintln!(
                    "   Tentativa {}/{}... ✅ Vite respondeu (status: {})",
                    i, max_retries, resp.status()
                );
                if resp.status() == 200 {
                    if let Ok(body) = resp.text().await {
                        if body.contains("<!doctype html") || body.contains("<div id=\"root\">") {
                            eprintln!("✅ Conteúdo válido: HTML do frontend detectado");
                            return Ok(());
                        } else if body.contains("Error") || body.contains("404") || body.contains("502") {
                            eprintln!("⚠️  Conteúdo parece ser erro: {}", body.lines().next().unwrap_or("erro"));
                        }
                    }
                }
                eprintln!("⚠️  Conteúdo da resposta não parece ser HTML válido");
                return Ok(());
            }
            Err(e) => {
                eprintln!("   Tentativa {}/{}... ⏳ aguardando ({})", i, max_retries, e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
    Err(format!("timeout após {} tentativas", max_retries))
}

fn main() {
    eprintln!("🚀 Harness Proxy — Modo Servidor");
    if let Err(e) = start_server_headless() {
        eprintln!("❌ Erro ao iniciar servidor: {}", e);
        std::process::exit(1);
    }
}

fn start_server_headless() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("📦 Iniciando Vite (frontend)...");
    let mut vite = Command::new("yarn")
        .args(&["--cwd", "frontend", "dev"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| format!("Erro ao iniciar Vite: {}. yarn está instalado?", e))?;

    dotenv().ok();

    let litellm_url = env::var("GATEWAY_URL")
        .unwrap_or_else(|_| {
            eprintln!("🔗 Gateway URL não definida, usando padrão: http://localhost:11435/v1");
            "http://localhost:11435/v1".to_string()
        });

    eprintln!("🔗 Gateway URL: {}", litellm_url);

    let rt = tokio::runtime::Runtime::new()?;

    eprintln!("🔍 Aguardando frontend em http://127.0.0.1:3528 ...");
    rt.block_on(check_frontend("http://127.0.0.1:3528", 30))
        .map_err(|e| {
            let _ = vite.kill();
            let _ = vite.wait();
            format!("Frontend não iniciou: {}", e)
        })?;

    let state = Arc::new(AppState::new(litellm_url.clone()));
    let app = Router::new()
        .route("/v1/models", get(handlers::models::list_models))
        .route("/v1/chat/completions", post(handlers::chat::intercept_chat))
        .route(
            "/v1/config/models",
            get(handlers::models::get_models_config)
                .post(handlers::models::update_models_config),
        )
        .fallback(handlers::proxy::transparent_proxy)
        .layer(build_cors_layer())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 11436));
    
    eprintln!("📡 Servidor Axum rodando em {}", addr);
    eprintln!("📋 Rotas disponíveis:");
    eprintln!("   GET    /v1/models");
    eprintln!("   POST   /v1/chat/completions");
    eprintln!("   GET    /v1/config/models");
    eprintln!("   POST   /v1/config/models");
    eprintln!("   *      /v1/* (proxy transparente)");
    eprintln!("✅ Servidor pronto. Pressione Ctrl+C para encerrar.");
    
    let axum_result = rt.block_on(async {
        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    });

    eprintln!("\n🛑 Encerrando servidores...");
    let _ = vite.kill();
    let _ = vite.wait();
    eprintln!("✅ Encerrado.");

    axum_result.map_err(|e| format!("Erro no servidor Axum: {}", e).into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_check_frontend_invalid_url() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(super::check_frontend("http://127.0.0.1:99999", 1));
        assert!(result.is_err(), "Deve falhar para porta invalida");
    }

    #[test]
    fn test_check_frontend_timeout() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(super::check_frontend("http://127.0.0.1:1", 2));
        assert!(result.is_err(), "Deve falhar para porta nao acessivel");
    }
}
