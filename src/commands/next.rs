use crate::{Context, Error};

#[allow(clippy::unused_async)]
#[poise::command(slash_command, subcommands("dota", "cs"))]
pub async fn next(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Next dota game for OG
#[poise::command(slash_command)]
pub async fn dota(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Failed to find guild")?;

    let mut event_list = guild_id.scheduled_events(&ctx, false).await?;
    if event_list.len() == 1 {
        if let Some(dota) = event_list.first() {
            ctx.reply(format!("https://discord.com/events/{guild_id}/{}", dota.id))
                .await?;
        }
    }

    event_list.sort_by(|a, b| a.start_time.timestamp().cmp(&b.start_time.timestamp()));
    let next_dota = event_list.iter().find(|event| event.name.contains("Dota"));
    if let Some(dota) = next_dota {
        ctx.reply(format!("https://discord.com/events/{guild_id}/{}", dota.id))
            .await?;
    } else {
        ctx.reply("No dota games planned currently").await?;
    }
    Ok(())
}

/// Next cs game for OG
#[poise::command(slash_command)]
pub async fn cs(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Failed to find guild")?;

    let mut event_list = guild_id.scheduled_events(&ctx, false).await?;
    if event_list.len() == 1 {
        if let Some(cs) = event_list.first() {
            ctx.reply(format!("https://discord.com/events/{guild_id}/{}", cs.id))
                .await?;
        }
    }

    event_list.sort_by(|a, b| a.start_time.timestamp().cmp(&b.start_time.timestamp()));
    let next_cs = event_list.iter().find(|event| event.name.contains("CS"));
    if let Some(cs) = next_cs {
        ctx.reply(format!("https://discord.com/events/{guild_id}/{}", cs.id))
            .await?;
    } else {
        ctx.reply("No dota games planned currently").await?;
    }
    Ok(())
}
