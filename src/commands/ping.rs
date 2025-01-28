use crate::{Context, Error};

/// Check the bot's gateway latency.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let shard_manager = ctx.framework.shard_manager;
    let latency = shard_manager
        .runners
        .lock()
        .await
        .get(&ctx.framework.serenity_context.shard_id)
        .ok_or("Failed to get shard")?
        .latency;

    if let Some(latency) = latency {
        ctx.reply(format!("Pong {latency:?}ms")).await?;
    } else {
        ctx.reply("Please wait for 1 minute to hear the bot heartbeat ")
            .await?;
    }

    Ok(())
}
