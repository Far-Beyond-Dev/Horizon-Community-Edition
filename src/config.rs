use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub players_per_pool: usize,
    pub num_thread_pools: usize,
    pub log_level: String,
}

impl Config {
    pub fn from_file(file_path: &str) -> Self {
        let config_str = fs::read_to_string(file_path)
            .expect("Failed to read configuration file");
        serde_yaml::from_str(&config_str)
            .expect("Failed to parse configuration file")
    }
}