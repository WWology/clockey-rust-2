use poise::serenity_prelude::EditRole;

use crate::{Context, Error};

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
