use std::time::Duration;
use tokio::time::interval;
use serenity::prelude::*;

use crate::db::cursedb::check_dumbname;

pub async fn ticker(ctx: Context ,secs: u64) {
    let mut interval = interval(Duration::from_secs(secs));
    interval.tick().await; // skip first tick

    loop {
        interval.tick().await;
        check_dumbname(ctx.clone()).await;
    }
}
