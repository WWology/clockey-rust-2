use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use poise::{
    serenity_prelude::{futures::future, CreateEmbed, CreateEmbedAuthor, Member, Timestamp},
    CreateReply,
};

use crate::{data::event::Event, Context, Error};

/// Generate invoice for a gardener from start_date to end_date
#[poise::command(slash_command)]
pub async fn invoice(
    ctx: Context<'_>,
    #[description = "Start date of the invoice, please use YYYY-MM-DD format"] start_date: String,
    #[description = "End date of the invoice, please use YYYY-MM-DD format"] end_date: Option<
        String,
    >,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let member = ctx.author_member().await.ok_or("Failed to find Member")?;

    let start = match NaiveDate::parse_from_str(start_date.as_str(), "%Y-%m-%d") {
        Ok(date) => date.and_time(NaiveTime::default()),
        Err(error) => {
            ctx.reply("Start date format is invalid, please use YYYY-MM-DD format")
                .await?;
            return Err(error.into());
        }
    };

    let end = match end_date {
        Some(end_date) => match NaiveDate::parse_from_str(end_date.as_str(), "%Y-%m-%d") {
            Ok(date) => date.and_time(NaiveTime::default()),
            Err(error) => {
                ctx.reply("End date format is invalid, please use YYYY-MM-DD format")
                    .await?;
                return Err(error.into());
            }
        },
        None => Utc::now().naive_utc(),
    };

    let gardener_id = ctx.author().id.get();
    let (dota_invoice, cs_invoice, rivals_invoice) = future::try_join3(
        Event::get_dota_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            i64::try_from(gardener_id)?,
        ),
        Event::get_cs_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            i64::try_from(gardener_id)?,
        ),
        Event::get_rivals_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            i64::try_from(gardener_id)?,
        ),
    )
    .await?;

    let (mlbb_invoice, hok_invoice, other_invoice) = future::try_join3(
        Event::get_mlbb_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            i64::try_from(gardener_id)?,
        ),
        Event::get_hok_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            i64::try_from(gardener_id)?,
        ),
        Event::get_other_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            i64::try_from(gardener_id)?,
        ),
    )
    .await?;

    let invoice_embed = generate_embed(
        dota_invoice,
        cs_invoice,
        rivals_invoice,
        mlbb_invoice,
        hok_invoice,
        other_invoice,
        start,
        end,
        &member,
    )?;
    ctx.send(CreateReply::new().embed(invoice_embed)).await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn generate_embed(
    dota_invoice: Vec<Event>,
    cs_invoice: Vec<Event>,
    rivals_invoice: Vec<Event>,
    mlbb_invoice: Vec<Event>,
    hok_invoice: Vec<Event>,
    other_invoice: Vec<Event>,
    start: NaiveDateTime,
    end: NaiveDateTime,
    member: &Member,
) -> Result<CreateEmbed, Error> {
    let (
        mut dota_events,
        mut cs_events,
        mut rivals_events,
        mut mlbb_events,
        mut hok_events,
        mut other_events,
    ): (String, String, String, String, String, String) = (
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    );
    let mut total_hours: i64 = 0;

    for event in dota_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        dota_events += format!(
            "{} at {} - {} hours\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours
        )
        .as_str();
        total_hours += event.hours;
    }

    for event in cs_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        cs_events += &format!(
            "{} at {} - {} hours\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours
        );
        total_hours += event.hours;
    }

    for event in rivals_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        rivals_events += &format!(
            "{} at {} - {} hours\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours
        );
        total_hours += event.hours;
    }

    for event in mlbb_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        mlbb_events += &format!(
            "{} at {} - {} hours\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours
        );
        total_hours += event.hours;
    }

    for event in hok_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        hok_events += &format!(
            "{} at {} - {} hours\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours
        );
        total_hours += event.hours;
    }

    for event in other_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        other_events += &format!(
            "{} at {} - {} hours\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours
        );
        total_hours += event.hours;
    }

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(member.display_name()).icon_url(member.face()))
        .title(format!("{} - {}", start.format("%B"), end.format("%B")))
        .fields(vec![
            ("Dota", dota_events, false),
            ("CS", cs_events, false),
            ("Rivals", rivals_events, false),
            ("MLBB", mlbb_events, false),
            ("HoK", hok_events, false),
            ("Other", other_events, false),
            ("Total Hours", total_hours.to_string(), false),
        ])
        .timestamp(Timestamp::now());
    Ok(embed)
}
