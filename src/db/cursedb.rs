use chrono::prelude::*;
use serenity::model::prelude::*;
use rusqlite::{Connection, Result};


pub async fn insert_curse(guildid: GuildId, userid: UserId, ts_uncurse: i64) -> Result<(), String> {
    let conn = match Connection::open("sqlite.db") {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to open database: {:?}",e)),
    };

    match conn.execute(
        "
        INSERT INTO curses (guild_id, user_id, time_uncurse) VALUES (?1, ?2, ?3)
        "
    ,(guildid.as_u64(), userid.as_u64(), ts_uncurse)) {
        Ok(_o) => Ok(()),
        Err(e) => Err(format!("Failed to insert into database: {:?}", e)),
    }
}

pub async fn check_curses() {
    
} 
