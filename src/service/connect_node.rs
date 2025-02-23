
use bitcoincore_rpc::{Auth, Client, RawTx, RpcApi};
use std::sync::Arc;

use crate::config::config::BitcoinNodeConfig;


pub fn connect_node(config: Arc<BitcoinNodeConfig>) -> Client {
    if config.url.is_empty() || config.rpc_user.is_empty() || config.rpc_password.is_empty(){
        panic!("Invalid configuration for Bitcoin Node");
    }
    let url = format!("http://{}:{}", config.url, config.port);

    Client::new(&url,  Auth::UserPass(config.rpc_user.clone(), config.rpc_password.clone())).unwrap()
}
    