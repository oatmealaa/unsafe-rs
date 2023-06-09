use std::env;

use serenity::{
    prelude::*,
};

use crate::handler::Handler;
use crate::db::db_init;

pub mod handler;
pub mod commands;
pub mod utils;
pub mod db;
pub mod tick;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let intents = GatewayIntents::all();

    db_init().await.unwrap(); 
     

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

}
