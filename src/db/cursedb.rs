use chrono::prelude::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::*;
use sqlx::Connection;
use std::collections::HashSet;
use serenity::futures::TryStreamExt;


#[derive(Debug)]
pub struct curse {
    guildid: String,
    userid: String,
    ts_uncurse: i64,
}

const DB_URL: &str = "sqlite://sqlite.db"; 

pub async fn insert_curse(guildid: GuildId, userid: UserId, ts_uncurse: i64) -> Result<()> {
    let mut conn = SqliteConnection::connect(DB_URL).await?;

    let (gid, uid) = (guildid.to_string(), userid.to_string());
    sqlx::query!("INSERT INTO curses (guild_id, user_id, time_uncurse) VALUES ($1, $2, $3)", gid, uid, ts_uncurse )
        .execute(&mut conn)
        .await?;

    Ok(())
}


pub async fn check_curses(ctx: Context) -> Result<()> {
    let mut conn = SqliteConnection::connect(DB_URL).await?;
    let mut conn2 = SqliteConnection::connect(DB_URL).await?; // dont ask
    let mut result = sqlx::query!("SELECT guild_id, user_id, time_uncurse FROM curses").fetch(&mut conn);
    

    let mut rows = vec![];
    while let Ok(Some(row)) = result.try_next().await {
        rows.push(curse {
            guildid: row.guild_id,
            userid: row.user_id,
            ts_uncurse: row.time_uncurse,
        })
    }

    for row in rows {

        let dt = Utc::now();
        if row.ts_uncurse >= dt.timestamp() {
            continue;
        }

        let gid: GuildId = GuildId(row.guildid.parse::<u64>().unwrap());
        let uid: UserId = UserId(row.userid.parse::<u64>().unwrap());

        let guild = match gid.to_guild_cached(&ctx) {
            Some(g) => g,
            None => panic!("no"),
        };

        let user = uid.to_user(&ctx).await.unwrap();

        match sqlx::query!("DELETE FROM curses WHERE guild_id=$1 AND user_id=$2 AND time_uncurse=$3",row.guildid, row.userid, row.ts_uncurse)
            .execute(&mut conn2)
            .await {
                Ok(m) => m,
                Err(why) => {println!("{:?}",why); continue;}
            };

        let member = guild.member(&ctx, &user).await.unwrap();
        let mut nick: String = match member.nick {
            Some(n) => n,
            None => continue,
        };
        
        let mut split: Vec<&str> = nick.split(" ").collect();
        
        let appendend: &str = "(dumb)";

        if split.get(split.len()-1).unwrap() == &appendend {
            split.remove(split.len()-1);
        }
        

        nick = split.join(" ");
        guild.edit_member(&ctx, &user, |m| m.nickname(nick)).await.unwrap();
        
        

    }

    Ok(())
}
