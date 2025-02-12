use std::vec;

use poise::{
    serenity_prelude::{
        ButtonStyle, ComponentInteractionCollector, CreateActionRow, CreateButton, Message,
    },
    CreateReply,
};
use regex::Regex;

use crate::{data::event::Event, Context, Error};

#[poise::command(context_menu_command = "Cancel Event")]
pub async fn cancel(ctx: Context<'_>, msg: Message) -> Result<(), Error> {
    if !message_processed(&ctx, &msg).await? {
        ctx.send(
            CreateReply::default()
                .content("This event has not been rolled yet")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let buttons = CreateActionRow::Buttons(vec![
        CreateButton::new("cancel_event_yes")
            .label("Yes")
            .style(ButtonStyle::Danger),
        CreateButton::new("cancel_event_no")
            .label("No")
            .style(ButtonStyle::Secondary),
    ]);

    let reply = ctx
        .send(
            CreateReply::new()
                .content("Are you sure you want to cancel signups for this event?")
                .components(vec![buttons]),
        )
        .await?;

    if get_confirmation(&ctx).await? {
        let (name, time) = get_event_name_and_time(msg.content.as_str())?;
        Event::delete(&ctx.data().db, name.as_str(), time).await?;
        reply
            .edit(
                poise::Context::from(ctx),
                CreateReply::new()
                    .content("Event cancelled")
                    .components(vec![]),
            )
            .await?;
    } else {
        reply
            .edit(
                poise::Context::from(ctx),
                CreateReply::new()
                    .content("Command cancelled")
                    .components(vec![]),
            )
            .await?;
    }
    Ok(())
}

/// Check if the message has been processed if signup emoji is present
async fn message_processed(ctx: &Context<'_>, msg: &Message) -> Result<bool, Error> {
    let processed_reactions = msg
        .reaction_users(
            ctx,
            ctx.data().config.processed_emoji.clone(),
            Some(1),
            None,
        )
        .await?;

    if processed_reactions.is_empty() {
        return Ok(false);
    }

    Ok(true)
}

fn get_event_name_and_time(msg: &str) -> Result<(String, i64), Error> {
    let name = Regex::new(r"-\s*(.*?)\s*,")?
        .captures(msg)
        .ok_or("Failed to find name")?
        .get(1)
        .ok_or("Failed to find name")?
        .as_str()
        .to_string();

    let unix_time = Regex::new(r"<t:(\d+):F>")?
        .captures(msg)
        .ok_or("Failed to find time")?
        .get(1)
        .ok_or("Failed to find time")?
        .as_str();
    let time = unix_time.trim().parse::<i64>()?;
    Ok((name, time))
}

async fn get_confirmation(ctx: &Context<'_>) -> Result<bool, Error> {
    if let Some(press) = ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with("cancel_event"))
        .timeout(std::time::Duration::from_secs(30))
        .await
    {
        if press.data.custom_id == "cancel_event_yes" {
            return Ok(true);
        } else if press.data.custom_id == "cancel_event_no" {
            return Ok(false);
        }
    }
    Err("Timed out".into())
}
