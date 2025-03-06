use poise::serenity_prelude::UserId;

use crate::{data::score, Context, Error};

#[allow(clippy::unused_async)]
#[poise::command(slash_command, subcommands("dota", "cs", "rivals"))]
pub async fn winners(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn dota(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    // Remove Dota Oracle role from current winners
    let current_holders: Vec<UserId> = {
        let guild_members = &ctx.guild().ok_or("Failed to find guild")?.members;

        guild_members
            .iter()
            .filter(|member| member.1.roles.contains(&ctx.data().config.dota_oracle_role))
            .map(|member| member.0.to_owned())
            .collect()
    };

    let guild_id = ctx.guild_id().ok_or("Failed to find guild")?;
    for holder in current_holders {
        ctx.http()
            .remove_member_role(guild_id, holder, ctx.data().config.dota_oracle_role, None)
            .await?;
    }

    // Get winners and add Dota Oracle role for new winners
    let winners = score::get_dota_winners(&ctx.data().db).await?;

    for winner in winners {
        ctx.http()
            .add_member_role(
                guild_id,
                UserId::new(u64::try_from(winner.id)?),
                ctx.data().config.dota_oracle_role,
                None,
            )
            .await?;
    }
    Ok(())
}

#[poise::command(slash_command)]
pub async fn cs(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    // Remove CS2 Awpacle role from current winners
    let current_holders: Vec<UserId> = {
        let guild_members = &ctx.guild().ok_or("Failed to find guild")?.members;

        guild_members
            .iter()
            .filter(|member| member.1.roles.contains(&ctx.data().config.cs2_awpacle_role))
            .map(|member| member.0.to_owned())
            .collect()
    };

    let guild_id = ctx.guild_id().ok_or("Failed to find guild")?;
    for holder in current_holders {
        ctx.http()
            .remove_member_role(guild_id, holder, ctx.data().config.cs2_awpacle_role, None)
            .await?;
    }

    // Get winners and add CS2 Awpacle role for new winners
    let winners = score::get_cs_winners(&ctx.data().db).await?;

    for winner in winners {
        ctx.http()
            .add_member_role(
                guild_id,
                UserId::new(u64::try_from(winner.id)?),
                ctx.data().config.cs2_awpacle_role,
                None,
            )
            .await?;
    }
    Ok(())
}

#[poise::command(slash_command)]
pub async fn rivals(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    // Remove Rivals Awpacle role from current winners
    let current_holders: Vec<UserId> = {
        let guild_members = &ctx.guild().ok_or("Failed to find guild")?.members;

        guild_members
            .iter()
            .filter(|member| {
                member
                    .1
                    .roles
                    .contains(&ctx.data().config.rivals_avengers_role)
            })
            .map(|member| member.0.to_owned())
            .collect()
    };

    let guild_id = ctx.guild_id().ok_or("Failed to find guild")?;
    for holder in current_holders {
        ctx.http()
            .remove_member_role(
                guild_id,
                holder,
                ctx.data().config.rivals_avengers_role,
                None,
            )
            .await?;
    }

    // Get winners and add Rivals Avenger role for new winners
    let winners = score::get_rivals_winners(&ctx.data().db).await?;

    for winner in winners {
        ctx.http()
            .add_member_role(
                guild_id,
                UserId::new(u64::try_from(winner.id)?),
                ctx.data().config.rivals_avengers_role,
                None,
            )
            .await?;
    }
    Ok(())
}
