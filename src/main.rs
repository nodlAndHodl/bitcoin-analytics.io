
mod api; 
mod repository;
mod config;
mod service;
use std::env;
use bitcoincore_rpc::{Client, RpcApi};
use config::config::AppConfig;
use ::config::File;
use repository::init_db;
use std::sync::Arc;
use service::connect_node::connect_node;
use ::config::Config;

#[macro_use]
extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    // Use environment variable or fallback


    let settings = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("Failed to build config");

    let config: AppConfig = settings.try_deserialize().expect("Failed to deserialize config");

    let db_url = env::var(config.database.url)
        .unwrap_or_else(|_| "sqlite://bitcoin_analytics.db".to_string());

    let config = Arc::new(config.bitcoin_node);

    let rcp_client: Client = connect_node(config);    
    
    let best_block =rcp_client.get_best_block_hash().unwrap();
    println!("Best block hash: {}", best_block);

    match init_db::init_db(&db_url).await {
        Ok(_) => println!("Database initialized successfully"),
        Err(e) => panic!("{}",e),
    }

    rocket::build()
        .mount("/", routes![world])    
}
