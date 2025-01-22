use poise::serenity_prelude::{
    CreateAllowedMentions, CreateScheduledEvent, ScheduledEventType, Timestamp,
};
use poise::{CreateReply, Modal};

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn event(
    ctx: Context<'_>,
    #[description = "The type of event"] event_type: EventChoice,
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

    match event_type {
        EventChoice::Dota => {
            let data = GameModal::execute(ctx).await?.ok_or("No data provided")?;
            name = format!("Dota - {}", data.name);
            time = data.time;
            hours = get_hours(data.series_length.as_str())?;
            series_length = Some(data.series_length);
            channel_id = if cfg!(debug_assertions) {
                738_607_620_566_286_398
            } else {
                super::DOTA_CHANNEL_ID
            };
            scheduled_type = ScheduledEventType::Voice;
        }
        EventChoice::CS => {
            let data = GameModal::execute(ctx).await?.ok_or("No data provided")?;
            name = format!("CS - {}", data.name);
            time = data.time;
            hours = get_hours(data.series_length.as_str())?;
            series_length = Some(data.series_length);
            channel_id = if cfg!(debug_assertions) {
                738_607_620_566_286_398
            } else {
                super::CS_CHANNEL_ID
            };
            scheduled_type = ScheduledEventType::Voice;
        }
        EventChoice::Other => {
            let data = EventModal::execute(ctx).await?.ok_or("No data provided")?;
            name = format!("Other - {}", data.name);
            time = data.time;
            hours = data.hours.parse::<u8>()?;
            series_length = None;
            channel_id = if cfg!(debug_assertions) {
                991_620_472_544_440_454
            } else {
                super::OG_STAGE_CHANNEL_ID
            };
            scheduled_type = ScheduledEventType::StageInstance;
        }
    }

    if let EventChoice::Other = event_type {
        reply_text = format!(
            "Hey <@&720253636797530203>\
            \n\nI need 1 gardener to work {name}, at <t:{time}:F>\
            \n\nPlease react below with a <:OGpeepoYes:730890894814740541> to sign up!\
            \n\nYou will be able to add {hours} hours of work to your invoice for the month"
        );
    } else {
        reply_text = format!(
            "Hey <@&720253636797530203>\
            \n\nI need 1 gardener to work {}, at <t:{}:F>\
            \n\nPlease react below with a <:OGpeepoYes:730890894814740541> to sign up!\
            \n\nAs this is a {}, you will be able to add {} hours of work to your invoice for the month",
            name,
            time,
            series_length.unwrap(),
            hours,
        );
    }

    let start_time = match time.parse::<i64>() {
        Ok(unix) => Timestamp::from_unix_timestamp(unix)?,
        Err(error) => {
            return Err(error.into());
        }
    };

    ctx.guild_id()
        .ok_or("error getting guild id")?
        .create_scheduled_event(
            &ctx,
            CreateScheduledEvent::new(scheduled_type, name, start_time).channel_id(channel_id),
        )
        .await?;

    let msg = if ping {
        ctx.say(reply_text).await?.into_message().await?
    } else {
        let reply = CreateReply::default().content(reply_text).allowed_mentions(
            CreateAllowedMentions::new()
                .all_roles(false)
                .all_users(false),
        );
        ctx.send(reply).await?.into_message().await?
    };

    msg.react(&ctx, ctx.data().signup_emoji.clone()).await?;
    Ok(())
}

#[derive(Debug, poise::ChoiceParameter)]
enum EventChoice {
    #[name = "Dota"]
    Dota,

    #[name = "CS"]
    CS,

    #[name = "Other"]
    Other,
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
