use poise::serenity_prelude::{EditMessage, MessageId};
use regex::Regex;

use crate::{Context, Error};

/// Edit the signup message
#[poise::command(slash_command)]
pub async fn edit(
    ctx: Context<'_>,
    #[description = "The message to be edited"] message_id: MessageId,
    #[description = "The new name for the event"] name: Option<String>,
    #[description = "The new time for the event"] time: Option<String>,
    #[description = "The new hours for the event"] hours: Option<String>,
) -> Result<(), Error> {
    let mut msg = ctx.http().get_message(ctx.channel_id(), message_id).await?;
    let mut new_message = msg.content.clone();

    if let Some(new_name) = name {
        let name_range = Regex::new(r"-\s*(.*?)\s*,")?
            .captures(new_message.as_str())
            .ok_or("Failed to find name")?
            .get(1)
            .ok_or("Failed to find name")?
            .range();
        new_message.replace_range(name_range, new_name.as_str());
    }

    if let Some(new_time) = time {
        let time_range = Regex::new(r"<t:(\d+):F>")?
            .captures(new_message.as_str())
            .ok_or("Failed to find time")?
            .get(1)
            .ok_or("Failed to find time")?
            .range();
        new_message.replace_range(time_range, new_time.as_str());
    }

    if let Some(new_hours) = hours {
        let hours_range = Regex::new(r"\badd\s*(\d)\s*")?
            .captures(new_message.as_str())
            .ok_or("Failed to find hours")?
            .get(1)
            .ok_or("Failed to find hours")?
            .range();
        new_message.replace_range(hours_range, new_hours.as_str());
    }

    msg.edit(&ctx, EditMessage::new().content(new_message.as_str()))
        .await?;
    ctx.reply("Updated the signup message").await?;
    Ok(())
}
