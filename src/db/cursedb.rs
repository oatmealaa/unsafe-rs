use chrono::prelude::*;
use serenity::model::prelude::*;
use rusqlite::{Connection, Result};
use serenity::prelude::*;


#[derive(Debug)]
pub struct curse {
    id: u64,
    guildid: String,
    userid: String,
    ts_uncurse: i64,
}

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


pub async fn check_curses(ctx: &Context) -> Result<()> {
    let conn = Connection::open("sqlite.db")?;

    
    
    
    let mut stmt = conn.prepare("SELECT id, guild_id, user_id, time_uncurse FROM curses")?;

    let curse_iter = stmt.query_map([], |row| {
    Ok(curse {
    id: row.get(0)?,
        guildid: row.get(1)?,
        userid: row.get(2)?,
        ts_uncurse: row.get(3)?,
    })
    })?;

    for _Curse in curse_iter {
        let Curse = _Curse.unwrap();
        let dt = Utc::now();
        if Curse.ts_uncurse >= dt.timestamp() {
            continue;
        }

        let gid: GuildId = GuildId(Curse.guildid.parse::<u64>().unwrap());
        let uid: UserId = UserId(Curse.guildid.parse::<u64>().unwrap());

        let guild = match gid.to_guild_cached(&ctx) {
            Some(g) => g,
            None => panic!("no guild found"), // !!!
        };

        let user = uid.to_user(&ctx).await.unwrap();

        let member = guild.member(&ctx, &user).await.unwrap();
        let mut nick: String = match member.nick {
            Some(n) => n,
            None => panic!("no member found"), // !!!
        };
        
        let mut split: Vec<&str> = nick.split(" ").collect();
        
        let appendend: &str = "(dumb)";

        if split.get(split.len()).unwrap() == &appendend {
            split.remove(split.len());
        }
        

        nick = split.join(" ");

        guild.edit_member(&ctx, &user, |m| m.nickname(nick)).await.unwrap();
    }
    

    Ok(())
}
