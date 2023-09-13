use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub avatar_dir: String,
    pub jwt_secret: String,
    pub addr_listen: String,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let file_path = "config.toml";
        let config_str = std::fs::read_to_string(file_path)
            .expect(&format!("Failed to read config file: {}", file_path));
        toml::from_str(&config_str).expect(&format!("Failed to parse config file: {}", file_path))
    };
}
