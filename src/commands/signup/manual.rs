use chrono::Utc;
use poise::{
    serenity_prelude::{CreateScheduledEvent, ScheduledEventType, Timestamp},
    Modal,
};

use crate::{
    data::event::{Event, EventType},
    Context, Error,
};

/// Manually add signups to an event
#[poise::command(slash_command)]
pub async fn manual(
    ctx: Context<'_>,
    #[description = "Type of the event"] event_choice: EventChoice,
    #[description = "Gardener to work on the event"] gardener: GardenerChoice,
) -> Result<(), Error> {
    let gardener_id: i64 = get_gardener_id(&gardener);

    let data = ManualModal::execute(ctx).await?.ok_or("No data provided")?;

    let start_time = match data.time.parse::<i64>() {
        Ok(unix) => Timestamp::from_unix_timestamp(unix)?,
        Err(error) => {
            return Err(error.into());
        }
    };
    let (name, event_type, scheduled_type, channel_id) = match event_choice {
        EventChoice::Dota => (
            format!("Dota - {}", &data.name),
            EventType::Dota,
            ScheduledEventType::Voice,
            ctx.data().config.dota_channel,
        ),
        EventChoice::CS => (
            format!("CS - {}", &data.name),
            EventType::CS,
            ScheduledEventType::Voice,
            ctx.data().config.cs_channel,
        ),
        EventChoice::Rivals => (
            format!("Rivals - {}", &data.name),
            EventType::Rivals,
            ScheduledEventType::Voice,
            ctx.data().config.rivals_channel,
        ),
        EventChoice::MLBB => (
            format!("MLBB - {}", &data.name),
            EventType::MLBB,
            ScheduledEventType::Voice,
            ctx.data().config.mlbb_channel,
        ),
        EventChoice::HoK => (
            format!("HoK - {}", &data.name),
            EventType::HoK,
            ScheduledEventType::Voice,
            ctx.data().config.hok_channel,
        ),
        EventChoice::Other => (
            format!("Other - {}", &data.name),
            EventType::Other,
            ScheduledEventType::StageInstance,
            ctx.data().config.stage_channel,
        ),
    };

    Event::new(
        data.name.as_str(),
        start_time.naive_utc(),
        event_type,
        gardener_id,
        data.hours.parse::<i64>()?,
    )
    .insert(&ctx.data().db)
    .await?;

    if Utc::now().timestamp() < start_time.timestamp() {
        ctx.guild_id()
            .ok_or("Error getting guild id")?
            .create_scheduled_event(
                ctx,
                CreateScheduledEvent::new(scheduled_type, &name, start_time).channel_id(channel_id),
            )
            .await?;
    }

    let msg = ctx
        .reply(format!(
            "The gardener working on {}, at <t:{}:F> is: <@{}>\
            \n\nYou can add {} hours of work to your invoice for the month",
            &name, &data.time, &gardener_id, &data.hours
        ))
        .await?
        .into_message()
        .await?;
    msg.react(&ctx, ctx.data().config.processed_emoji.clone())
        .await?;

    Ok(())
}

fn get_gardener_id(gardener: &GardenerChoice) -> i64 {
    match gardener {
        GardenerChoice::Nik => 293_360_731_867_316_225,
        GardenerChoice::Kit => 204_923_365_205_475_329,
        GardenerChoice::WW => 754_724_309_276_164_159,
        GardenerChoice::Bonteng => 172_360_818_715_918_337,
        GardenerChoice::Sam => 332_438_787_588_227_072,
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, poise::ChoiceParameter)]
enum EventChoice {
    #[name = "Dota"]
    Dota,

    #[name = "CS"]
    CS,

    #[name = "Rivals"]
    Rivals,

    #[name = "MLBB"]
    MLBB,

    #[name = "HoK"]
    HoK,

    #[name = "Other"]
    Other,
}

#[derive(Debug, poise::ChoiceParameter)]
enum GardenerChoice {
    #[name = "Nik"]
    Nik,

    #[name = "Kit"]
    Kit,

    #[name = "WW"]
    WW,

    #[name = "Bonteng"]
    Bonteng,

    #[name = "Sam"]
    Sam,
}

#[derive(Debug, Modal)]
#[name = "Event / Game Information"]
struct ManualModal {
    #[name = "Game / Event name"]
    #[placeholder = "OG vs <opp team name>"]
    name: String,

    #[name = "Game / Event time"]
    #[placeholder = "Insert Unix time here"]
    time: String,

    #[name = "Hours"]
    #[placeholder = "How many hours is this event"]
    hours: String,
}
