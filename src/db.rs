use std::path::Path;
use std::fs::File;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use sqlx::*;

pub mod cursedb;

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn db_init() -> Result<()> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        Sqlite::create_database(DB_URL).await.unwrap();
    }
    
    let mut conn = SqliteConnection::connect(DB_URL).await?;

    sqlx::query!("CREATE TABLE IF NOT EXISTS curses ( guild_id TEXT NOT NULL, user_id TEXT NOT NULL, time_uncurse INTEGER NOT NULL);")
        .execute(&mut conn)
        .await?;

    Ok(()) 
}
