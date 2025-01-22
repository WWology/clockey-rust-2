use std::time::Duration;

use chrono::NaiveDateTime;
use poise::{serenity_prelude::*, CreateReply};
use regex::Regex;

use crate::{
    data::event::{Event, EventType},
    Context, Error,
};

#[poise::command(context_menu_command = "Roll Gardener")]
pub async fn gardener(ctx: Context<'_>, msg: Message) -> Result<(), Error> {
    if message_processed(&ctx, &msg).await? {
        ctx.send(
            CreateReply::default()
                .content("This message has been processed for signups")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let gardener_select_menu = gardener_select_menu_builder(&ctx, &msg).await?;
    let message = ctx
        .send(
            CreateReply::default()
                .content("Choose the gardener to work this event")
                .components(vec![CreateActionRow::SelectMenu(gardener_select_menu)])
                .ephemeral(true),
        )
        .await?
        .into_message()
        .await?;

    let gardener_working = match get_gardener_working(&ctx, &message).await {
        Ok(gardener) => gardener,
        Err(error) => {
            ctx.say("Something wrong has happened, please try again")
                .await?;
            return Err(error);
        }
    };

    let (name, time, event_type, hours) = parse_message(msg.content.as_str())?;
    Event::new(name.as_str(), time, event_type, gardener_working, hours)
        .insert(&ctx.data().db)
        .await?;

    message
        .react(&ctx, ctx.data().processed_emoji.clone())
        .await?;
    ctx.interaction
        .edit_response(
            &ctx,
            EditInteractionResponse::default()
                .content("Hours added to the database")
                .components(vec![]),
        )
        .await?;
    ctx.channel_id()
        .send_message(
            &ctx,
            CreateMessage::new().reference_message(
                MessageReference::new(MessageReferenceKind::Forward, ctx.channel_id())
                    .message_id(msg.id),
            ),
        )
        .await?;
    ctx.channel_id()
        .send_message(
            &ctx,
            CreateMessage::new().content(format!(
                "The gardener working on {name} is <@{gardener_working}>"
            )),
        )
        .await?;
    Ok(())
}

async fn message_processed(ctx: &Context<'_>, msg: &Message) -> Result<bool, Error> {
    let processed_reactions = msg
        .reaction_users(ctx, ctx.data().processed_emoji.clone(), Some(1), None)
        .await?;

    if processed_reactions.is_empty() {
        return Ok(false);
    }

    Ok(true)
}

async fn gardener_select_menu_builder(
    ctx: &Context<'_>,
    msg: &Message,
) -> Result<CreateSelectMenu, Error> {
    let clockey_id = ctx.interaction.application_id.get();

    let gardeners_reacted = msg
        .reaction_users(ctx, ctx.data().signup_emoji.clone(), Some(6), None)
        .await?;

    let ids: Vec<u64> = gardeners_reacted
        .iter()
        .filter(|gardeners| gardeners.id.get() != clockey_id)
        .map(|gardeners| gardeners.id.get())
        .collect();

    let mut gardener_select_menu_options = vec![];

    for id in ids {
        match id {
            293_360_731_867_316_225 => {
                gardener_select_menu_options.push(CreateSelectMenuOption::new("Nik", "Nik"));
            }
            204_923_365_205_475_329 => {
                gardener_select_menu_options.push(CreateSelectMenuOption::new("Kit", "Kit"));
            }
            754_724_309_276_164_159 => {
                gardener_select_menu_options.push(CreateSelectMenuOption::new("WW", "WW"));
            }
            172_360_818_715_918_337 => {
                gardener_select_menu_options
                    .push(CreateSelectMenuOption::new("Bonteng", "Bonteng"));
            }
            332_438_787_588_227_072 => {
                gardener_select_menu_options.push(CreateSelectMenuOption::new("Sam", "Sam"));
            }
            _ => {
                return Err("Invalid gardener ID".into());
            }
        }
    }

    let gardener_select_menu = CreateSelectMenu::new(
        "gardener_select",
        CreateSelectMenuKind::String {
            options: gardener_select_menu_options,
        },
    );
    Ok(gardener_select_menu)
}

async fn get_gardener_working(ctx: &Context<'_>, msg: &Message) -> Result<i64, Error> {
    let Some(interaction) = msg
        .await_component_interaction(&ctx.serenity_context().shard)
        .timeout(Duration::from_secs(60 * 2))
        .await
    else {
        msg.reply(ctx, "Timed out").await?;
        return Err("Timed out".into());
    };

    let gardener_selected = match &interaction.data.kind {
        ComponentInteractionDataKind::StringSelect { values } => &values[0],
        _ => return Err("Invalid interaction data kind".into()),
    };

    let gardener_working: i64 = match gardener_selected.as_str() {
        "Nik" => 293_360_731_867_316_225,
        "Kit" => 204_923_365_205_475_329,
        "WW" => 754_724_309_276_164_159,
        "Bonteng" => 172_360_818_715_918_337,
        "Sam" => 332_438_787_588_227_072,
        _ => {
            return Err("Invalid gardener selected".into());
        }
    };
    Ok(gardener_working)
}

type EventDetail = (String, NaiveDateTime, EventType, i64);

fn parse_message(msg: &str) -> Result<EventDetail, Error> {
    let event_type = if msg.contains("Dota") {
        EventType::Dota
    } else if msg.contains("CS") {
        EventType::CS
    } else if msg.contains("Other") {
        EventType::Other
    } else {
        return Err("Invalid event type".into());
    };

    let name = Regex::new(r"-\s*(.*?)\s*,")?
        .captures(msg)
        .ok_or("Failed to find name")?
        .get(1)
        .ok_or("Failed to find name")?
        .as_str();

    let unix_time = Regex::new(r"<t:(\d+):F>")?
        .captures(msg)
        .ok_or("Failed to find time")?
        .get(1)
        .ok_or("Failed to find time")?
        .as_str();
    let time = Timestamp::from_unix_timestamp(unix_time.trim().parse::<i64>()?)?.naive_utc();

    let hours = Regex::new(r"\badd\s*(\d)\s*")?
        .captures(msg)
        .ok_or("Failed to find hours")?
        .get(1)
        .ok_or("Failed to find hours")?
        .as_str()
        .parse::<i64>()?;

    Ok((name.to_string(), time, event_type, hours))
}
