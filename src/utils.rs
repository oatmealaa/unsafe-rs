use serenity::{
    prelude::*,
    model::prelude::*,
    utils::{parse_channel, parse_emoji, parse_role, parse_username},
};

use rand::prelude::*;

pub async fn parse_user(name: String, guildid: Option<&GuildId>, ctx: &Context) -> Option<UserId> {
    
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
    
    if let Some(x) =  guild.member_named(&name) {
        return Some(x.user.id);
    }

    if let Some(m) = guild.members_containing(&name, false, true).await.get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    }

    if let Some(m) = guild.members_starting_with(&name, false, true).await.get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    }
    
    None
}

pub async fn random_user(guildid: Option<&GuildId>, ctx: &Context) -> Option<UserId> {

    let gid = match guildid {
        Some(g) => g,
        None => {
            println!("No guildid");
            return None;
        },
    };

    let guild = match gid.to_guild_cached(&ctx)     {
        Some(x) => x,
        None => {
            println!("Failure getting guild");
            return None;
        },
    };

    let members = guild.members(&ctx, None, None).await.expect("curse.rs: Failure retriveing member count");   
    let mut randomindex: usize = 0;
    
    {
        let mut rng = rand::thread_rng();
        randomindex = rng.gen_range(0..members.len());
    }
     
    match members.get(randomindex) {
        Some(u) => {
            return Some(u.user.id);
        },
        None => {
            println!("Failed to get random user");
            return None;
        },
    }
}
