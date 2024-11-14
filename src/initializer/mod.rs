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