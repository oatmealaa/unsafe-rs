use rusqlite::{Connection, Result};
use std::path::Path;
use std::fs::File;

pub mod cursedb;

pub async fn db_init() -> Result<(), String> {
    let conn = match Connection::open("sqlite.db") {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to open db: {:?}",e)),
    };

    match conn.execute_batch(
        "
        BEGIN;
        CREATE TABLE IF NOT EXISTS curses (id INTEGER PRIMARY KEY, guild_id TEXT NOT NULL, user_id TEXT NOT NULL, time_uncurse INTEGER NOT NULL);

        COMMIT;
        "
    ) {
        Ok(o) => Ok(()),
        Err(e) => Err(format!("Failed to setup db: {:?}",e)),
    }
}
