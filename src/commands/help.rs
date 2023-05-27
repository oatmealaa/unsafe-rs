use serenity::{
    model::channel::Message,
    prelude::*,
};

const HELP_MESSAGE: &str = "
Commands:

**!curse**
    Gives member a random curse. If no member is provided the command will pick a random member in the guild to curse.

    Example:
    !curse gamer123
";

pub async fn help_msg(ctx: Context, msg: Message) {
    msg.reply(&ctx,HELP_MESSAGE).await.expect("help.rs line 12");
    
}
