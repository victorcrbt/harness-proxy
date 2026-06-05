use reqwest::Client;

pub struct AppState {
    pub client: Client,
    pub litellm_url: String,
}

impl AppState {
    pub fn new(litellm_url: String) -> Self {
        Self {
            client: Client::new(),
            litellm_url,
        }
    }
}
