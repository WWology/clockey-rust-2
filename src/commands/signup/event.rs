use poise::serenity_prelude::{
    CreateAllowedMentions, CreateScheduledEvent, ScheduledEventType, Timestamp,
};
use poise::{CreateReply, Modal};

use crate::{data::event::EventType, Context, Error};

/// Create a new event for Gardeners to sign up for
#[poise::command(slash_command)]
#[allow(clippy::too_many_lines)]
pub async fn event(
    ctx: Context<'_>,
    #[description = "The type of event"] event_type: EventType,
    #[description = "Ping gardeners or not"] ping: Option<bool>,
) -> Result<(), Error> {
    let reply_text: String;

    let name: String;
    let time: String;
    let series_length: Option<String>;
    let hours: u8;
    let channel_id: u64;
    let scheduled_type: ScheduledEventType;
    let ping = ping.unwrap_or(true);

    // Execute Game Modal for Dota and CS Events
    // Execute Event Modal for Other Events
    match event_type {
        EventType::Dota => {
            let data = GameModal::execute(ctx).await?.ok_or("No data provided")?;
            name = format!("Dota - {}", data.name);
            time = data.time;
            hours = get_hours(data.series_length.as_str())?;
            series_length = Some(data.series_length);
            channel_id = ctx.data().config.dota_channel;
            scheduled_type = ScheduledEventType::Voice;
        }
        EventType::CS => {
            let data = GameModal::execute(ctx).await?.ok_or("No data provided")?;
            name = format!("CS - {}", data.name);
            time = data.time;
            hours = get_hours(data.series_length.as_str())?;
            series_length = Some(data.series_length);
            channel_id = ctx.data().config.cs_channel;
            scheduled_type = ScheduledEventType::Voice;
        }
        EventType::Rivals => {
            let data = GameModal::execute(ctx).await?.ok_or("No data provided")?;
            name = format!("Rivals - {}", data.name);
            time = data.time;
            hours = get_rivals_hours(data.series_length.as_str())?;
            series_length = Some(data.series_length);
            channel_id = ctx.data().config.rivals_channel;
            scheduled_type = ScheduledEventType::Voice;
        }
        EventType::MLBB => {
            let data = GameModal::execute(ctx).await?.ok_or("No data provided")?;
            name = format!("MLBB - {}", data.name);
            time = data.time;
            hours = get_mlbb_hok_hours(data.series_length.as_str())?;
            series_length = Some(data.series_length);
            channel_id = ctx.data().config.mlbb_channel;
            scheduled_type = ScheduledEventType::Voice;
        }
        EventType::HoK => {
            let data = GameModal::execute(ctx).await?.ok_or("No data provided")?;
            name = format!("HoK - {}", data.name);
            time = data.time;
            hours = get_mlbb_hok_hours(data.series_length.as_str())?;
            series_length = Some(data.series_length);
            channel_id = ctx.data().config.hok_channel;
            scheduled_type = ScheduledEventType::Voice;
        }
        EventType::Other => {
            let data = EventModal::execute(ctx).await?.ok_or("No data provided")?;
            name = format!("Other - {}", data.name);
            time = data.time;
            hours = data.hours.parse::<u8>()?;
            series_length = None;
            channel_id = ctx.data().config.stage_channel;
            scheduled_type = ScheduledEventType::StageInstance;
        }
    }

    // Reply text contains series length if it's a game event
    if let EventType::Other = event_type {
        reply_text = format!(
            "Hey <@&720253636797530203>\
            \n\nI need 1 gardener to work {name}, at <t:{time}:F> <t:{time}:R>\
            \n\nPlease react below with a <:OGpeepoYes:730890894814740541> to sign up!\
            \n\nYou will be able to add {hours} hours of work to your invoice for the month"
        );
    } else {
        reply_text = format!(
            "Hey <@&720253636797530203>\
            \n\nI need 1 gardener to work {}, at <t:{}:F> <t:{}:R>\
            \n\nPlease react below with a <:OGpeepoYes:730890894814740541> to sign up!\
            \n\nAs this is a {}, you will be able to add {} hours of work to your invoice for the month",
            name,
            time,
            time,
            series_length.expect("Series length missing"),
            hours,
        );
    }

    // Try to parse time passed from modal to a unix timestamp
    let start_time = match time.parse::<i64>() {
        Ok(unix) => Timestamp::from_unix_timestamp(unix)?,
        Err(error) => {
            return Err(error.into());
        }
    };

    // Create Discord Scheduled Event
    ctx.guild_id()
        .ok_or("Failed to find guild")?
        .create_scheduled_event(
            &ctx,
            CreateScheduledEvent::new(scheduled_type, name, start_time).channel_id(channel_id),
        )
        .await?;

    // Ping Gardeners or not depending on ping flag
    let msg = if ping {
        let reply = CreateReply::new()
            .content(reply_text)
            .allowed_mentions(CreateAllowedMentions::new().all_roles(true).all_users(true));
        ctx.send(reply).await?.into_message().await?
    } else {
        let reply = CreateReply::new().content(reply_text).allowed_mentions(
            CreateAllowedMentions::new()
                .all_roles(false)
                .all_users(false),
        );
        ctx.send(reply).await?.into_message().await?
    };

    // React to the message with the signup emoji
    msg.react(&ctx, ctx.data().config.signup_emoji.clone())
        .await?;
    Ok(())
}

#[derive(Debug, Modal)]
#[name = "Game Information"]
struct GameModal {
    #[name = "Game name"]
    #[placeholder = "OG vs <opp team name>"]
    name: String,

    #[name = "Game time"]
    #[placeholder = "Insert unix time here"]
    time: String,

    #[name = "Game series length"]
    #[placeholder = "Bo1 / Bo2 / Bo3 / Bo5"]
    series_length: String,
}

#[derive(Debug, Modal)]
#[name = "Event Information"]
struct EventModal {
    #[name = "Event name"]
    #[placeholder = "OG vs <opp team name>"]
    name: String,

    #[name = "Event time"]
    #[placeholder = "Insert unix time here"]
    time: String,

    #[name = "How many hours is this event?"]
    hours: String,
}

fn get_hours(series_length: &str) -> Result<u8, Error> {
    match series_length.to_lowercase().as_str() {
        "bo1" => Ok(2),
        "bo2" => Ok(3),
        "bo3" => Ok(4),
        "bo5" => Ok(6),
        _ => Err("Invalid series length".into()),
    }
}

fn get_rivals_hours(series_length: &str) -> Result<u8, Error> {
    match series_length.to_lowercase().as_str() {
        "bo3" => Ok(3),
        "bo5" => Ok(4),
        "bo7" => Ok(5),
        _ => Err("Invalid series length".into()),
    }
}

fn get_mlbb_hok_hours(series_length: &str) -> Result<u8, Error> {
    match series_length.to_lowercase().as_str() {
        "bo3" => Ok(2),
        "bo5" => Ok(3),
        "bo7" => Ok(4),
        _ => Err("Invalid series length".into()),
    }
}
