use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub addr_listen: String,

    pub static_dir: String,
    pub image_dir: String,
    pub avatar_dir: String,

    pub static_url: String,

    pub avatar_width: u32,
    pub avatar_mini_width: u32,
    pub avatar_quality: u8,

    pub jwt_secret: String,

    pub exp_days: i64,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let file_path = "config.toml";
        let config_str = std::fs::read_to_string(file_path)
            .expect(&format!("Failed to read config file: {}", file_path));
        toml::from_str(&config_str).expect(&format!("Failed to parse config file: {}", file_path))
    };
}
