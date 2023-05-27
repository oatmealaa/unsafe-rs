use serenity::{
    model::channel::Message,
    prelude::*,
};

pub mod help;
pub use help::*;

pub mod curse;
pub use curse::*;

pub async fn command(ctx: Context,msg: Message) {
    let split: Vec<&str> = msg.content.split(" ").collect();
    


    match split[0] {
        "!help" => help::help_msg(ctx,msg).await,
        "!curse" => curse::curse(ctx,msg).await,
        _ => (),
    }
}
