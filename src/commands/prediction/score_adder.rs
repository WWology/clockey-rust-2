use poise::serenity_prelude::Role;

use crate::{data::score, Context, Error};

/// Add scores to the Dota scoreboard for the winning role
#[poise::command(slash_command)]
pub async fn dotaadd(ctx: Context<'_>, role: Role) -> Result<(), Error> {
    ctx.defer().await?;

    let winners: Vec<u64> = {
        let guild_members = &ctx.guild().ok_or("Failed to find guild")?.members;

        guild_members
            .iter()
            .filter(|member| member.1.roles.contains(&role.id))
            .map(|member| member.0.get())
            .collect()
    };

    let winners_count = winners.len();
    for id in winners {
        score::update_dota_scoreboard(&ctx.data().db, id).await?;
    }

    ctx.reply(format!(
        "Added score for {winners_count} users to the Dota scoreboard",
    ))
    .await?;
    Ok(())
}

/// Add scores to the CS scoreboard for the winning role
#[poise::command(slash_command)]
pub async fn csadd(ctx: Context<'_>, role: Role) -> Result<(), Error> {
    ctx.defer().await?;

    let winners: Vec<u64> = {
        let guild_members = &ctx.guild().ok_or("Failed to find guild")?.members;

        guild_members
            .iter()
            .filter(|member| member.1.roles.contains(&role.id))
            .map(|member| member.0.get())
            .collect()
    };

    let winners_count = winners.len();
    for id in winners {
        score::update_cs_scoreboard(&ctx.data().db, id).await?;
    }

    ctx.reply(format!(
        "Added score for {winners_count} users to the CS scoreboard",
    ))
    .await?;
    Ok(())
}

/// Add scores to the Rivals scoreboard for the winning role
#[poise::command(slash_command)]
pub async fn rivalsadd(ctx: Context<'_>, role: Role) -> Result<(), Error> {
    ctx.defer().await?;

    let winners: Vec<u64> = {
        let guild_members = &ctx.guild().ok_or("Failed to find guild")?.members;

        guild_members
            .iter()
            .filter(|member| member.1.roles.contains(&role.id))
            .map(|member| member.0.get())
            .collect()
    };

    let winners_count = winners.len();
    for id in winners {
        score::update_rivals_scoreboard(&ctx.data().db, id).await?;
    }

    ctx.reply(format!(
        "Added score for {winners_count} users to the Rivals scoreboard",
    ))
    .await?;
    Ok(())
}

/// Add scores to the MLBB scoreboard for the winning role
#[poise::command(slash_command)]
pub async fn mlbbadd(ctx: Context<'_>, role: Role) -> Result<(), Error> {
    ctx.defer().await?;

    let winners: Vec<u64> = {
        let guild_members = &ctx.guild().ok_or("Failed to find guild")?.members;

        guild_members
            .iter()
            .filter(|member| member.1.roles.contains(&role.id))
            .map(|member| member.0.get())
            .collect()
    };

    let winners_count = winners.len();
    for id in winners {
        score::update_mlbb_scoreboard(&ctx.data().db, id).await?;
    }

    ctx.reply(format!(
        "Added score for {winners_count} users to the MLBB scoreboard",
    ))
    .await?;
    Ok(())
}

/// Add scores to the HoK scoreboard for the winning role
#[poise::command(slash_command)]
pub async fn hokadd(ctx: Context<'_>, role: Role) -> Result<(), Error> {
    ctx.defer().await?;

    let winners: Vec<u64> = {
        let guild_members = &ctx.guild().ok_or("Failed to find guild")?.members;

        guild_members
            .iter()
            .filter(|member| member.1.roles.contains(&role.id))
            .map(|member| member.0.get())
            .collect()
    };

    let winners_count = winners.len();
    for id in winners {
        score::update_hok_scoreboard(&ctx.data().db, id).await?;
    }

    ctx.reply(format!(
        "Added score for {winners_count} users to the HoK scoreboard",
    ))
    .await?;
    Ok(())
}
