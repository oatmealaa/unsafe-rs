use std::time::Duration;
use tokio::{
    select,
    task::spawn,
    time::{interval, sleep},
};
use serenity::prelude::*;

use crate::db::cursedb::check_curses;

pub async fn ticker(ctx: Context ,secs: u64) {
    let mut interval = interval(Duration::from_secs(secs));
    interval.tick().await; // skip first tick

    loop {
        interval.tick().await;

        if let Err(why) = check_curses(&ctx).await {
            panic!("{}",why);
        } 
    }
}
