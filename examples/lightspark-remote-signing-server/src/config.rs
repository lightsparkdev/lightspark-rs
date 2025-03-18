#[derive(Debug, Clone)]
pub struct Config {
    pub api_endpoint: Option<String>,
    pub api_client_id: String,
    pub api_client_secret: String,
    pub webhook_secret: String,
    pub master_seed_hex: String,
    pub api_port: u16,
    pub respond_directly: bool,
}

impl Config {
    pub fn new_from_env() -> Self {
        let api_endpoint = std::env::var("API_ENDPOINT").ok();
        let api_client_id = std::env::var("API_CLIENT_ID").ok();
        let api_client_secret = std::env::var("API_CLIENT_SECRET").ok();
        let webhook_secret = std::env::var("WEBHOOK_SECRET").ok();
        let master_seed_hex = std::env::var("MASTER_SEED_HEX").ok();
        let api_port = std::env::var("PORT").ok();
        let respond_directly = std::env::var("RESPOND_DIRECTLY").is_ok();

        Self {
            api_endpoint,
            api_client_id: api_client_id.unwrap_or_default(),
            api_client_secret: api_client_secret.unwrap_or_default(),
            webhook_secret: webhook_secret.unwrap_or_default(),
            master_seed_hex: master_seed_hex.unwrap_or_default(),
            api_port: api_port
                .unwrap_or("8080".to_string())
                .parse()
                .expect("api port"),
            respond_directly,
        }
    }
}
