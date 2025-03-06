use poise::serenity_prelude::EditRole;

use crate::{Context, Error};

/// Create Dota roles for predictions
#[poise::command(slash_command)]
pub async fn dotabo(ctx: Context<'_>, series_length: u8) -> Result<(), Error> {
    ctx.defer().await?;

    let guild = ctx.guild_id().ok_or("Failed to find guild")?;

    match series_length {
        1 => {
            let roles_to_be_created = ["D1-0", "D0-1"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a Dota Bo1").await?;
        }
        2 => {
            let roles_to_be_created = ["D2-0", "D1-1", "D0-2"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a Dota Bo2").await?;
        }
        3 => {
            let roles_to_be_created = ["D2-0", "D2-1", "D1-2", "D0-2"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a Dota Bo3").await?;
        }
        5 => {
            let roles_to_be_created = ["D3-0", "D3-1", "D3-2", "D2-3", "D1-3", "D0-3"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a Dota Bo5").await?;
        }
        _ => {
            ctx.reply("Invalid series length").await?;
        }
    }

    Ok(())
}

/// Create CS roles for predictions
#[poise::command(slash_command)]
pub async fn csbo(ctx: Context<'_>, series_length: u8) -> Result<(), Error> {
    ctx.defer().await?;

    let guild = ctx.guild_id().ok_or("Failed to find guild")?;

    match series_length {
        1 => {
            let roles_to_be_created = ["CS1-0", "CS0-1"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a CS Bo1").await?;
        }
        2 => {
            let roles_to_be_created = ["CS2-0", "CS1-1", "CS0-2"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a CS Bo2").await?;
        }
        3 => {
            let roles_to_be_created = ["CS2-0", "CS2-1", "CS1-2", "CS0-2"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a CS Bo3").await?;
        }
        5 => {
            let roles_to_be_created = ["CS3-0", "CS3-1", "CS3-2", "CS2-3", "CS1-3", "CS0-3"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a CS Bo5").await?;
        }
        _ => {
            ctx.reply("Invalid series length").await?;
        }
    }

    Ok(())
}

/// Create Rivals roles for predictions
#[poise::command(slash_command)]
pub async fn rivalsbo(ctx: Context<'_>, series_length: u8) -> Result<(), Error> {
    ctx.defer().await?;

    let guild = ctx.guild_id().ok_or("Failed to find guild")?;

    match series_length {
        1 => {
            let roles_to_be_created = ["RIV1-0", "RIV0-1"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a Rivals Bo1").await?;
        }
        2 => {
            let roles_to_be_created = ["RIV2-0", "RIV1-1", "RIV0-2"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a Rivals Bo2").await?;
        }
        3 => {
            let roles_to_be_created = ["RIV2-0", "RIV2-1", "RIV1-2", "RIV0-2"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a Rivals Bo3").await?;
        }
        5 => {
            let roles_to_be_created = ["RIV3-0", "RIV3-1", "RIV3-2", "RIV2-3", "RIV1-3", "RIV0-3"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a Rivals Bo5").await?;
        }
        7 => {
            let roles_to_be_created = [
                "RIV4-0", "RIV4-1", "RIV4-2", "RIV4-3", "RIV3-4", "RIV2-4", "RIV1-4", "RIV0-4",
            ];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for a Rivals Bo7").await?;
        }
        _ => {
            ctx.reply("Invalid series length").await?;
        }
    }

    Ok(())
}

/// Create extra roles for predictions
#[poise::command(slash_command)]
pub async fn extrabo(ctx: Context<'_>, series_length: u8) -> Result<(), Error> {
    ctx.defer().await?;

    let guild = ctx.guild_id().ok_or("Failed to find guild")?;

    match series_length {
        1 => {
            let roles_to_be_created = ["EX1-0", "EX0-1"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for an extra Bo1").await?;
        }
        2 => {
            let roles_to_be_created = ["EX2-0", "EX1-1", "EX0-2"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for an extra Bo2").await?;
        }
        3 => {
            let roles_to_be_created = ["EX2-0", "EX2-1", "EX1-2", "EX0-2"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for an extra Bo3").await?;
        }
        5 => {
            let roles_to_be_created = ["EX3-0", "EX3-1", "EX3-2", "EX2-3", "EX1-3", "EX0-3"];
            for role_name in roles_to_be_created {
                guild
                    .create_role(&ctx, EditRole::new().name(role_name))
                    .await?;
            }
            ctx.reply("Created roles for an extra Bo5").await?;
        }
        _ => {
            ctx.reply("Invalid series length").await?;
        }
    }

    Ok(())
}

/// Delete created Dota roles for prediction
#[poise::command(slash_command)]
pub async fn deletedota(ctx: Context<'_>, series_length: u8) -> Result<(), Error> {
    ctx.defer().await?;

    let guild = ctx.guild_id().ok_or("Failed to find guild")?;
    let role_list = guild.roles(&ctx).await?;

    match series_length {
        1 => {
            let roles_to_be_deleted = ["D1-0", "D0-1"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for Dota Bo1").await?;
        }
        2 => {
            let roles_to_be_deleted = ["D2-0", "D1-1", "D0-1"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for Dota Bo2").await?;
        }
        3 => {
            let roles_to_be_deleted = ["D2-0", "D2-1", "D1-2", "D0-2"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for Dota Bo3").await?;
        }
        5 => {
            let roles_to_be_deleted = ["D3-0", "D3-1", "D3-2", "D2-3", "D1-3", "D0-3"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for Dota Bo5").await?;
        }
        _ => {
            ctx.reply("Invalid series length").await?;
        }
    }

    Ok(())
}

/// Delete created CS roles for prediction
#[poise::command(slash_command)]
pub async fn deletecs(ctx: Context<'_>, series_length: u8) -> Result<(), Error> {
    ctx.defer().await?;

    let guild = ctx.guild_id().ok_or("Failed to find guild")?;
    let role_list = guild.roles(&ctx).await?;

    match series_length {
        1 => {
            let roles_to_be_deleted = ["CS1-0", "CS0-1"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for CS Bo1").await?;
        }
        2 => {
            let roles_to_be_deleted = ["CS2-0", "CS1-1", "CS0-1"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for CS Bo2").await?;
        }
        3 => {
            let roles_to_be_deleted = ["CS2-0", "CS2-1", "CS1-2", "CS0-2"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for CS Bo3").await?;
        }
        5 => {
            let roles_to_be_deleted = ["CS3-0", "CS3-1", "CS3-2", "CS2-3", "CS1-3", "CS0-3"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for CS Bo5").await?;
        }
        _ => {
            ctx.reply("Invalid series length").await?;
        }
    }

    Ok(())
}

/// Delete created Rivals roles for prediction
#[poise::command(slash_command)]
pub async fn deleterivals(ctx: Context<'_>, series_length: u8) -> Result<(), Error> {
    ctx.defer().await?;

    let guild = ctx.guild_id().ok_or("Failed to find guild")?;
    let role_list = guild.roles(&ctx).await?;

    match series_length {
        1 => {
            let roles_to_be_deleted = ["RIV1-0", "RIV0-1"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for Rivals Bo1").await?;
        }
        2 => {
            let roles_to_be_deleted = ["RIV2-0", "RIV1-1", "RIV0-1"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for Rivals Bo2").await?;
        }
        3 => {
            let roles_to_be_deleted = ["RIV2-0", "RIV2-1", "RIV1-2", "RIV0-2"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for Rivals Bo3").await?;
        }
        5 => {
            let roles_to_be_deleted = ["RIV3-0", "RIV3-1", "RIV3-2", "RIV2-3", "RIV1-3", "RIV0-3"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for Rivals Bo5").await?;
        }
        7 => {
            let roles_to_be_deleted = [
                "RIV4-0", "RIV4-1", "RIV4-2", "RIV4-3", "RIV3-4", "RIV2-4", "RIV1-4", "RIV0-4",
            ];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for Rivals Bo7").await?;
        }
        _ => {
            ctx.reply("Invalid series length").await?;
        }
    }

    Ok(())
}

/// Delete created extra roles for prediction
#[poise::command(slash_command)]
pub async fn deleteextra(ctx: Context<'_>, series_length: u8) -> Result<(), Error> {
    ctx.defer().await?;

    let guild = ctx.guild_id().ok_or("Failed to find guild")?;
    let role_list = guild.roles(&ctx).await?;

    match series_length {
        1 => {
            let roles_to_be_deleted = ["EX1-0", "EX0-1"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for extra Bo1").await?;
        }
        2 => {
            let roles_to_be_deleted = ["EX2-0", "EX1-1", "EX0-1"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for extra Bo2").await?;
        }
        3 => {
            let roles_to_be_deleted = ["EX2-0", "EX2-1", "EX1-2", "EX0-2"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for extra Bo3").await?;
        }
        5 => {
            let roles_to_be_deleted = ["EX3-0", "EX3-1", "EX3-2", "EX2-3", "EX1-3", "EX0-3"];
            for role_name in roles_to_be_deleted {
                if let Some(role) = role_list
                    .iter()
                    .find(|role| role.1.name.as_str() == role_name)
                {
                    guild.delete_role(&ctx, role.0).await?;
                }
            }
            ctx.reply("Deleted roles for extra Bo5").await?;
        }
        _ => {
            ctx.reply("Invalid series length").await?;
        }
    }

    Ok(())
}
