
mod api; 
mod repository;

use std::env;
use repository::init_db;

#[macro_use]
extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    // Use environment variable or fallback
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://bitcoin_analytics.db".to_string());

    match init_db::init_db(&db_url).await {
        Ok(_) => println!("Database initialized successfully"),
        Err(e) => panic!("{}",e),
    }
    rocket::build()
        .mount("/", routes![world])    
}
