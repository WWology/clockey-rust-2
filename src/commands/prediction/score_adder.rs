use poise::serenity_prelude::Role;

use crate::{Context, Error, data::score};

/// Add scores to the Dota scoreboard for the winning role
#[poise::command(slash_command)]
pub async fn dotaadd(ctx: Context<'_>, role: Role) -> Result<(), Error> {
    ctx.defer().await?;

    let winners: Vec<u64>;
    {
        let guild_members = &ctx.guild().ok_or("Failed to find guild")?.members;

        winners = guild_members
            .iter()
            .filter(|member| member.1.roles.contains(&role.id))
            .map(|member| member.0.get())
            .collect();
    }

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
        "Added score for {winners_count} users to the Dota scoreboard",
    ))
    .await?;
    Ok(())
}
