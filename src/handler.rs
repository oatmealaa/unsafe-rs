 use serenity::{
     async_trait,
     model::{channel::Message, gateway::Ready},
     prelude::*,
 };
use crate::commands;
use crate::tick::ticker;

use tokio::{
    select,
    task::spawn,
    time::{interval, sleep},
};


pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with("!") {
            return;
        }
        commands::command(ctx,msg).await;
    }


    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
            let tick_1s = spawn(ticker(ctx.clone(),60));
    }
}
