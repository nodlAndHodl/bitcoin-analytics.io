use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct BitcoinNodeConfig {
    pub url: String,
    pub port: u16,  
    pub rpc_user: String,
    pub rpc_password: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub rpc_timeout: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub bitcoin_node: BitcoinNodeConfig,
}