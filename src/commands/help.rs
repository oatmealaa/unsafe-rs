use serenity::{
    async_trait,
    model::channel::Message,
    prelude::*,
};

const HELP_MESSAGE: &str = "
help message
";

pub async fn help_msg(ctx: Context, msg: Message) {
    msg.reply(&ctx,HELP_MESSAGE).await.expect("help.rs line 12");
    
}
