use crate::{data, Context, Error};

#[allow(clippy::unused_async)]
#[poise::command(slash_command, subcommands("dota", "cs", "rivals", "mlbb", "hok"))]
pub async fn reset(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Reset Dota scoreboard
#[poise::command(slash_command)]
pub async fn dota(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    data::score::reset_dota_scoreboard(&ctx.data().db).await?;
    ctx.reply("Dota scoreboard reset successful").await?;
    Ok(())
}

/// Reset CS scoreboard
#[poise::command(slash_command)]
pub async fn cs(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    data::score::reset_cs_scoreboard(&ctx.data().db).await?;
    ctx.reply("CS scoreboard reset successful").await?;
    Ok(())
}

/// Reset Rivals scoreboard
#[poise::command(slash_command)]
pub async fn rivals(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    data::score::reset_rivals_scoreboard(&ctx.data().db).await?;
    ctx.reply("Rivals scoreboard reset successful").await?;
    Ok(())
}

/// Reset MLBB scoreboard
#[poise::command(slash_command)]
pub async fn mlbb(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    data::score::reset_mlbb_scoreboard(&ctx.data().db).await?;
    ctx.reply("MLBB scoreboard reset successful").await?;
    Ok(())
}

/// Reset HoK scoreboard
#[poise::command(slash_command)]
pub async fn hok(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    data::score::reset_hok_scoreboard(&ctx.data().db).await?;
    ctx.reply("HoK scoreboard reset successful").await?;
    Ok(())
}
