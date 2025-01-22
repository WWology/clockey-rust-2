use std::time::Instant;

use poise::CreateReply;

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();
    let msg = ctx.reply("Pinging...").await?;
    let latency = start.elapsed().as_millis();
    msg.edit(
        poise::Context::Application(ctx),
        CreateReply::default().content(format!("Pong! {latency}ms")),
    )
    .await?;

    Ok(())
}
