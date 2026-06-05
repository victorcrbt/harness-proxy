use reqwest::Client;
use std::{fs, path::Path};

pub struct AppState {
    pub client: Client,
    pub litellm_url: String,
}

impl AppState {
    pub fn new(litellm_url: String) -> Self {
        if !Path::new("models.json").exists() {
            if let Err(e) = fs::write("models.json", "[]") {
                eprintln!("⚠️  Não foi possível criar models.json: {}", e);
            } else {
                eprintln!("📄 models.json criado com lista vazia");
            }
        }

        Self {
            client: Client::new(),
            litellm_url,
        }
    }
}
