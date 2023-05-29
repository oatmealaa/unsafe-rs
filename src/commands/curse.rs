use crate::utils;
use serenity::{
    model::channel::Message,
    prelude::*,
};
use chrono::Utc;

use rand::prelude::*;
use crate::db::cursedb::insert_curse;

const SECONDS_IN_HALF_HOUR: i64 = 1800;

#[derive(Debug)]
pub enum CurseType {
    DumbName,
    NoThe,
    NoEmoji,
}

pub async fn curse(ctx: Context, msg: Message) {
    let split: Vec<&str> = msg.content.split(" ").collect();
    let mut curse: i32;

    {
        let mut rng = rand::thread_rng();
        curse = rng.gen_range(0..=1);
    }
    
    let mut args: Vec<&str> = vec![];

    for (i,arg) in split.iter().enumerate() {
        if i==0 {continue;}            
        args.push(arg);
    }
    
    match curse {
        0 => stinky_name(&ctx, &msg, args).await,
        1 => no_the(&ctx, &msg, args).await,
        _ => (),
    };
}

async fn stinky_name(ctx: &Context,msg: &Message, args: Vec<&str>) {
    let guild_id = match msg.guild_id {
        Some(x) => x,
        None => return,
    };
    
    println!("0");
    let guild = match guild_id.to_guild_cached(&ctx) {
        Some(g) => g,
        None => return,
    };
    println!("1");

    let user_id = match args.get(0) {
         Some(u) => utils::parse_user(u.to_string(), Some(&guild_id), &ctx).await.expect("nope"),
         None => utils::random_user(Some(&guild_id), &ctx).await.expect("Failed to generate ramdom user from guild"),
    };
    
    let user = user_id.to_user(&ctx).await.expect("nope");

    let member = match guild.member(&ctx, &user).await {
        Ok(m) => m,
        Err(why) => {
            panic!("{:?}", why);
        },
    };
    
    msg.channel_id.say(&ctx, &format!("{} has been cursed with dumb name for 30 minutes!", &member.user.name)).await;

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
    
    guild.edit_member(&ctx, &user, |m| m.nickname(nick)).await.expect("Couldent edit members nickname");

    let dt = Utc::now();
    insert_curse(guild_id, user_id, dt.timestamp()+SECONDS_IN_HALF_HOUR, CurseType::DumbName).await.unwrap();
}

async fn no_the(ctx: &Context, msg: &Message, args: Vec<&str>) {
    let guild_id = match msg.guild_id {
        Some(x) => x,
        None => return,
    };

    let user_id = match args.get(0) {
         Some(u) => utils::parse_user(u.to_string(), Some(&guild_id), &ctx).await.expect("User parse failed"),
         None => utils::random_user(Some(&guild_id), &ctx).await.expect("Failed to generate ramdom user from guild"),
    };

    let user = user_id.to_user(&ctx).await.expect("Failed converting user id to user");

    let guild = match guild_id.to_guild_cached(&ctx) {
        Some(g) => g,
        None => return,
    };

    let member = match guild.member(&ctx, &user).await {
        Ok(m) => m,
        Err(why) => {
            panic!("{:?}", why);
        },
    };

    let dt = Utc::now();

    msg.channel_id.say(&ctx, &format!("{} can not use the word **the** for 30 minutes", &member.user.name)).await;
    insert_curse(guild_id, user_id, dt.timestamp()+SECONDS_IN_HALF_HOUR, CurseType::NoThe).await.unwrap();
    println!("inserted no the");
}

async fn no_emoji() {

}

