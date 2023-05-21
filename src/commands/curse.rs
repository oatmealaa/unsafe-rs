use crate::utils;
use serenity::{
    async_trait,
    model::channel::Message,
    prelude::*,
};

use rand::prelude::*;

pub async fn curse(ctx: Context, msg: Message) {
    let split: Vec<&str> = msg.content.split(" ").collect();
    let mut curse: i32 = 0;

    {
        let mut rng = rand::thread_rng();
        curse = rng.gen_range(0..=2);
    }

    let arg: Option<&str> = split.get(1).copied();

    match curse {
        0 => stinky_name(ctx, &msg, arg).await,
        _ => ()
    }
}

async fn stinky_name(ctx: Context,msg: &Message, op_user: Option<&str>) {
    let guild_id = msg.guild_id;
    

    let user = match op_user {
         Some(u) => utils::parse_user(u, Some(&guild_id), &ctx).await,
         None => utils::random_user(Some(&guild_id), &ctx).await
    };

}


