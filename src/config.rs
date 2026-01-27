use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::{Result, Context};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalSettings {
    pub use_color: bool,
    pub use_compression: bool,
    pub compression_level: u32,
    pub use_encryption: bool,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            use_color: true,
            use_compression: true,
            compression_level: 6,
            use_encryption: true,
        }
    }
}

impl GlobalSettings {
    pub fn load() -> Self {
        dirs::home_dir().and_then(|mut p| {
            p.push(".config/tokenizer/settings.toml");
            fs::read_to_string(p).ok()
        }).and_then(|s| toml::from_str(&s).ok()).unwrap_or_default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LocalConfig {
    pub token_type: String,
    pub method: String,
    pub target: String,
    pub headers_json: String,
    pub timeout_ms: u64,
    pub body_data: String,
    pub payload: String,
    pub output_file: String,
}

impl LocalConfig {
    pub fn init() -> Result<()> {
        let content = r#"token_type = "WebToken"
method = "GET"
target = "https://httpbin.org/get"
headers_json = '{"Accept": "application/json"}'
timeout_ms = 5000
body_data = ""
payload = ""
output_file = "test.token""#;
        fs::write("config.toml", content).context("Failed to create local config.toml")
    }

    pub fn load() -> Result<Self> {
        let content = fs::read_to_string("config.toml")
            .context("Local config.toml not found. Run 'tetra config.init' first.")?;
        toml::from_str(&content).context("Failed to parse config.toml")
    }
}
