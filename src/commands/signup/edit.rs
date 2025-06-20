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
    let mut reply_message = String::from("Updated event details: \n");
    if let Some(new_name) = name {
        let name_match = Regex::new(r"-\s*(.*?)\s*,")?
            .captures(new_message.as_str())
            .ok_or("Failed to find name")?
            .get(1)
            .ok_or("Failed to find name")?;
        reply_message.push_str(format!("{} to {new_name}\n", name_match.as_str()).as_str());
        let name_range = name_match.range();
        new_message.replace_range(name_range, new_name.as_str());
    }

    if let Some(new_time) = time {
        let time_match = Regex::new(r"<t:(\d+):F>")?
            .captures(new_message.as_str())
            .ok_or("Failed to find time")?
            .get(1)
            .ok_or("Failed to find time")?;
        reply_message
            .push_str(format!("<t:{}:F> to <t:{new_time}:F>\n", time_match.as_str()).as_str());
        let time_range = time_match.range();
        new_message.replace_range(time_range, new_time.as_str());
    }

    if let Some(new_hours) = hours {
        let hours_match = Regex::new(r"\badd\s*(\d)\s*")?
            .captures(new_message.as_str())
            .ok_or("Failed to find hours")?
            .get(1)
            .ok_or("Failed to find hours")?;
        reply_message.push_str(format!("{} to {new_hours}\n", hours_match.as_str()).as_str());
        let hours_range = hours_match.range();
        new_message.replace_range(hours_range, new_hours.as_str());
    }

    msg.edit(&ctx, EditMessage::new().content(new_message.as_str()))
        .await?;

    ctx.reply(reply_message).await?;
    Ok(())
}
