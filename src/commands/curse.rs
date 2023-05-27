use crate::utils;
use serenity::{
    async_trait,
    model::channel::Message,
    prelude::*,
};
use chrono::{DateTime, Utc};

use rand::prelude::*;
use crate::db::cursedb::insert_curse;

const SECONDS_IN_HALF_HOUR: i64 = 1800;

pub async fn curse(ctx: Context, msg: Message) {
    let split: Vec<&str> = msg.content.split(" ").collect();
    let mut curse: i32 = 0;

    {
        let mut rng = rand::thread_rng();
        //curse = rng.gen_range(0..=2);
    }

    let arg: Option<&str> = split.get(1).copied();
    match curse {
        0 => stinky_name(ctx, &msg, arg).await,
        _ => ()
    }
}

async fn stinky_name(ctx: Context,msg: &Message, op_user: Option<&str>) {
    let guild_id = match msg.guild_id {
        Some(x) => x,
        None => return,
    };
    
    let guild = match guild_id.to_guild_cached(&ctx) {
        Some(g) => g,
        None => return,
    };

    let user_id = match op_user {
         Some(u) => utils::parse_user(u.to_string(), Some(&guild_id), &ctx).await.expect("nope"),
         None => utils::random_user(Some(&guild_id), &ctx).await.expect("nope")
    };
    
    
    println!("{:?}", user_id); 

    let user = user_id.to_user(&ctx).await.expect("nope");

    let member = match guild.member(&ctx, &user).await {
        Ok(m) => m,
        Err(why) => {
            panic!("{:?}", why);
        },
    };

    msg.channel_id.say(&ctx, &format!("{} has been cursed with dumb name for 30 minutes!", &member.user.name)).await;
    

    let dt = Utc::now();

    
    match &member.nick {
        Some(n) => {
            let split: Vec<&str> = n.split(" ").collect();
            if split[split.len()-1] == "(dumb)" {
                return;
            }
        }
        None => ()
    }
    

    let mut nick: String = match member.nick {
        Some(n) => n,
        None => member.user.name,
    };

    nick.push_str(" (dumb)");

    if user.bot == true {

    }
    
    guild.edit_member(&ctx, &user, |m| m.nickname(nick)).await.expect("err");
    insert_curse(guild_id, user_id, dt.timestamp()+SECONDS_IN_HALF_HOUR).await.unwrap();
}

async fn no_the() {

}

async fn no_emoji() {

}

