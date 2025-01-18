use crate::{Context, Error};
use log::{log, Level};
use poise::serenity_prelude::{
    CreateAllowedMentions, CreateScheduledEvent, EmojiId, ReactionType, ScheduledEventType,
    Timestamp,
};
use poise::{CreateReply, Modal};

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
            let data = GameModal::execute(ctx)
                .await?
                .ok_or_else(|| "No data provided")?;
            name = format!("Dota - {}", data.name);
            time = data.time;
            hours = get_hours(data.series_length.as_str())?;
            series_length = Some(data.series_length);
            // channel_id = 738_009_797_932_351_519;
            channel_id = 738_607_620_566_286_398;
            scheduled_type = ScheduledEventType::Voice;
        }
        EventChoice::CS => {
            let data = GameModal::execute(ctx)
                .await?
                .ok_or_else(|| "No data provided")?;
            name = format!("CS - {}", data.name);
            time = data.time;
            hours = get_hours(data.series_length.as_str())?;
            series_length = Some(data.series_length);
            // channel_id = 746_618_267_434_614_804;
            channel_id = 738_607_620_566_286_398;
            scheduled_type = ScheduledEventType::Voice;
        }
        EventChoice::Other => {
            let data = EventModal::execute(ctx)
                .await?
                .ok_or_else(|| "No data provided")?;
            name = format!("Other - {}", data.name);
            time = data.time;
            hours = data.hours.parse::<u8>()?;
            series_length = None;
            // channel_id = 1_186_593_338_300_842_025;
            channel_id = 991_620_472_544_440_454;
            scheduled_type = ScheduledEventType::StageInstance;
        }
    }

    if let EventChoice::Other = event_type {
        reply_text = format!(
            "Hey <@&720253636797530203>\
            \n\nI need 1 gardener to work the {}, at <t:{}:F>\
            \n\nPlease react below with a <:OGpeepoYes:730890894814740541> to sign up!\
            \n\nYou will be able to add {} hours of work to your invoice for the month",
            name, time, hours
        );
    } else {
        reply_text = format!(
            "Hey <@&720253636797530203>\
            \n\nI need 1 gardener to work the {}, at <t:{}:F>\
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
            log!(Level::Error, "Error parsing time string to i64: {}", time);
            return Err(error.into());
        }
    };
    let event = CreateScheduledEvent::new(scheduled_type, name, start_time).channel_id(channel_id);

    ctx.guild_id()
        .ok_or_else(|| "error getting guild id")?
        .create_scheduled_event(ctx, event)
        .await?;

    // let og_peepo_yes_emoji = ReactionType::Custom {
    //     animated: false,
    //     id: EmojiId::new(730_890_894_814_740_541),
    //     name: Some(String::from("OGpeepoYes")),
    // };

    let reactor_emoji = ReactionType::Custom {
        animated: false,
        id: EmojiId::new(951843834554376262),
        name: Some(String::from("ruggahPain")),
    };

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

    msg.react(&ctx, reactor_emoji).await?;
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

#[derive(Debug, poise::ChoiceParameter)]
enum EventChoice {
    #[name = "Dota"]
    Dota,

    #[name = "CS"]
    CS,

    #[name = "Other"]
    Other,
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
