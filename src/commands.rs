use serenity::{
    async_trait,
    model::channel::Message,
    prelude::*,
};

pub mod help;
pub use help::*;

pub mod curse;
pub use curse::*;

pub async fn command(ctx: Context,msg: Message) {
    match msg.content.as_str() {
        "!help" => help::help_msg(ctx,msg).await,
        _ => (),
    }
}
