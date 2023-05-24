 use serenity::{
     async_trait,
     model::{channel::Message, gateway::Ready},
     prelude::*,
 };
use crate::commands;
use crate::tick::ticker;


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
        ticker(ct   x ,10).await;
    }
}
