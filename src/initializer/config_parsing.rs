use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InitDrone {
    pub id: u64,
    pub connected_drone_ids: Vec<u64>,
    pub pdr: f64,
}

#[derive(Debug, Deserialize)]
pub struct InitClient {
    pub id: u64,
    pub connected_drone_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct InitServer {
    pub id: u64,
    pub connected_drone_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct InitConfig {
    pub drone: Vec<InitDrone>,
    pub client: Vec<InitClient>,
    pub server: Vec<InitServer>,
}

pub fn parse_config(config_path: String) -> InitConfig {
    let config_data = fs::read_to_string("src/config.toml").expect("Unable to read config file");

    toml::from_str(&config_data).expect("Unable to parse TOML")
}