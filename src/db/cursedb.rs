use chrono::prelude::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::*;
use sqlx::Connection;
use crate::commands::curse::CurseType;
use serenity::futures::TryStreamExt;

#[derive(Debug)]
pub struct Curse {
    guildid: String,
    userid: String,
    ts_uncurse: i64,
curse_type: CurseType,
}

const DB_URL: &str = "sqlite://sqlite.db"; 

pub async fn insert_curse(guildid: GuildId, userid: UserId, ts_uncurse: i64, cursetype: CurseType) -> Result<()> {
    let mut conn = SqliteConnection::connect(DB_URL).await?;

    let (gid, uid) = (guildid.to_string(), userid.to_string());
    let cursetypedb: i64 = match cursetype {
        CurseType::DumbName => 0,
        CurseType::NoThe => 1,
        _ => {panic!("Supplied curse type is not a valid curse type");}
    };
    sqlx::query!("INSERT INTO curses (guild_id, user_id, time_uncurse, curse_type) VALUES ($1, $2, $3, $4)", gid, uid, ts_uncurse, cursetypedb)
        .execute(&mut conn)
        .await?;

    Ok(())
}


pub async fn check_dumbname(ctx: Context) -> Result<()> {
    let mut conn = SqliteConnection::connect(DB_URL).await?;
    let mut conn2 = SqliteConnection::connect(DB_URL).await?; // dont ask
    let mut result = sqlx::query!("SELECT guild_id, user_id, time_uncurse, curse_type FROM curses").fetch(&mut conn);
    

    let mut rows = vec![];
    while let Ok(Some(row)) = result.try_next().await {
        rows.push(Curse {
            guildid: row.guild_id,
            userid: row.user_id,
            ts_uncurse: row.time_uncurse,
            curse_type: match row.curse_type {
                0 => CurseType::DumbName,
                1 => CurseType::NoThe,
                _ => {panic!("Invalid curse type in database");},
            },
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
        let cursetypedb: i64 = match row.curse_type {
            CurseType::DumbName => 0,
            CurseType::NoThe => 1,
            _ => {panic!("Supplied curse type is not a valid curse type");}
        };
        match sqlx::query!("DELETE FROM curses WHERE guild_id=$1 AND user_id=$2 AND time_uncurse=$3 AND curse_type=$4",row.guildid, row.userid, row.ts_uncurse, cursetypedb)
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

pub async fn Check_for_the(msg: &Message, ctx: &Context) {
    let mut conn = SqliteConnection::connect(DB_URL).await.unwrap();

    let split: Vec<&str> = msg.content.split(" ").collect();

    let mut has_the: bool = false;
    for word in split {
        if word.contains("the") || word.contains("The") || word.contains("tHe") || word.contains("thE") || word.contains("THe") || word.contains("ThE") || word.contains("tHE") || word.contains("THE") {
            has_the = true;
            break;
        }
    }

    if has_the == false {
        return;
    }

    let (gid, uid) = (msg.guild_id.unwrap().to_string(), msg.author.id.to_string());
    let mut result = sqlx::query!("SELECT curse_type FROM curses WHERE guild_id=$1 AND user_id=$2 AND curse_type=1", gid, uid)
        .fetch(&mut conn);
   
    if let Ok(Some(row)) = result.try_next().await {
        msg.delete(&ctx).await.unwrap();
    }
}

pub async fn Check_for_emoji() {

}
