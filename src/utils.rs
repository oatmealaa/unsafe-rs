use serenity::{
    prelude::*,
    model::prelude::*,
    utils::{parse_channel, parse_emoji, parse_role, parse_username},
};

use rand::prelude::*;

pub async fn parse_user(name: &str, guildid: Option<GuildId>, ctx: &Context) -> Option<UserId> {
    
    if let Some(x) = parse_username(&name) {
        return Some(UserId(x));
    }
    
    let gid = match guildid {
        Some(g) => g,
        None => return None,
    };

    let guild = match gid.to_guild_cached(&ctx) {
        Some(x) => x,
        None => return None,
    };
    
    if let Some(x) =  guild.members_named(&name) {
        return Some(x.user.id);
    }

    if let Some(m) = guild.members_containing(&name, false, true).await.get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    }

    if let Some(m) = guild.mmbers_starting_with(&name, false, true).await.get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    }
    
    None
}

pub async fn random_user(guildid: Option<&GuildId>, ctx: &Context) -> Option<UserId> {

    let gid = match guildid {
        Some(g) => g,
        None => return None,
    };

    let guild = match gid.to_guild_cached(&ctx) {
        Some(x) => x,
        None => return None,
    };

    let mut rng = rand::thread_rng();
    
    let members = guild.members(&ctx).await.expect("curse.rs: Failure retriveing member count");
    
    members.get(rng.gen_range(..members.len()))
}
