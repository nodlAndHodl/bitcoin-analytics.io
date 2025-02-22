use sqlx::{migrate::{MigrateDatabase, Migrator}, Sqlite,SqlitePool};
use std::{error::Error, path::Path};

pub async fn init_db(url: &str) -> Result<(), Box<dyn Error>> {
    if !Sqlite::database_exists(&url).await.unwrap_or(false) {
        Sqlite::create_database(&url).await.unwrap();
    }

    let pool = SqlitePool::connect(&url).await?;
    match run_migrations(&pool).await {
        Ok(_) => println!("Database created Sucessfully"),
        Err(e) => panic!("{}",e),
    }

    Ok(())
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    let migrator = Migrator::new(Path::new("./src/repository/migrations")).await?;
    migrator.run(pool).await?;
    Ok(())
}