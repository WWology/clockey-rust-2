use chrono::Utc;
use poise::{
    Modal,
    serenity_prelude::{CreateScheduledEvent, ScheduledEventType, Timestamp},
};

use crate::{
    Context, Error,
    data::event::{Event, EventType},
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
        EventChoice::Dota => {
            if cfg!(debug_assertions) {
                (
                    format!("Dota - {}", &data.name),
                    EventType::Dota,
                    ScheduledEventType::Voice,
                    738_607_620_566_286_398,
                )
            } else {
                (
                    format!("Dota - {}", &data.name),
                    EventType::Dota,
                    ScheduledEventType::Voice,
                    super::DOTA_CHANNEL_ID,
                )
            }
        }
        EventChoice::CS => {
            if cfg!(debug_assertions) {
                (
                    format!("CS - {}", &data.name),
                    EventType::CS,
                    ScheduledEventType::Voice,
                    738_607_620_566_286_398,
                )
            } else {
                (
                    format!("CS - {}", &data.name),
                    EventType::CS,
                    ScheduledEventType::Voice,
                    super::CS_CHANNEL_ID,
                )
            }
        }
        EventChoice::Other => {
            if cfg!(debug_assertions) {
                (
                    format!("Other - {}", &data.name),
                    EventType::Other,
                    ScheduledEventType::StageInstance,
                    991_620_472_544_440_454,
                )
            } else {
                (
                    format!("Other - {}", &data.name),
                    EventType::Other,
                    ScheduledEventType::StageInstance,
                    super::OG_STAGE_CHANNEL_ID,
                )
            }
        }
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
    msg.react(&ctx, ctx.data().processed_emoji.clone()).await?;

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

#[derive(Debug, poise::ChoiceParameter)]
enum EventChoice {
    #[name = "Dota"]
    Dota,

    #[name = "CS"]
    CS,

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
