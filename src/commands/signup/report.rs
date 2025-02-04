use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use poise::{
    serenity_prelude::{futures::future, CreateEmbed, CreateEmbedAuthor, Member, Timestamp},
    CreateReply,
};

use crate::{data::event::Event, Context, Error};

#[poise::command(slash_command)]
pub async fn report(
    ctx: Context<'_>,
    #[description = "Start date of the invoice, please use YYYY-MM-DD format"] start_date: String,
    #[description = "End date of the invoice, please use YYYY-MM-DD format"] end_date: Option<
        String,
    >,
    #[description = "Gardener to get a report from"] gardener: Option<Member>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

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

    if let Some(gardener) = gardener {
        let report = generate_report_for_gardener(&ctx, start, end, gardener).await?;
        ctx.send(CreateReply::new().embed(report)).await?;
    } else {
        let report = generate_report(&ctx, start, end).await?;
        ctx.send(CreateReply::new().embed(report)).await?;
    }
    Ok(())
}

async fn generate_report_for_gardener(
    ctx: &Context<'_>,
    start: NaiveDateTime,
    end: NaiveDateTime,
    gardener: Member,
) -> Result<CreateEmbed, Error> {
    let (dota_invoice, cs_invoice, other_invoice) = future::try_join3(
        Event::get_dota_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            i64::try_from(gardener.user.id.get())?,
        ),
        Event::get_cs_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            i64::try_from(gardener.user.id.get())?,
        ),
        Event::get_other_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            i64::try_from(gardener.user.id.get())?,
        ),
    )
    .await?;

    let (mut dota_events, mut cs_events, mut other_events): (String, String, String) =
        (String::new(), String::new(), String::new());
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
        .author(CreateEmbedAuthor::new(gardener.display_name()).icon_url(gardener.face()))
        .title(format!("{} - {}", start.format("%B"), end.format("%B")))
        .fields(vec![
            ("Dota", dota_events, false),
            ("CS", cs_events, false),
            ("Other", other_events, false),
            ("Total Hours", total_hours.to_string(), false),
        ])
        .timestamp(Timestamp::now());
    Ok(embed)
}

async fn generate_report(
    ctx: &Context<'_>,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<CreateEmbed, Error> {
    let (dota_invoice, cs_invoice, other_invoice) = future::try_join3(
        Event::get_dota_events(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
        ),
        Event::get_cs_events(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
        ),
        Event::get_other_events(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
        ),
    )
    .await?;

    let (mut dota_events, mut cs_events, mut other_events): (String, String, String) =
        (String::new(), String::new(), String::new());
    let mut total_hours: i64 = 0;

    for event in dota_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        dota_events += format!(
            "{} at {} - {} hours by {}\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours,
            event.gardener_name(),
        )
        .as_str();
        total_hours += event.hours;
    }

    for event in cs_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        cs_events += &format!(
            "{} at {} - {} hours by {}\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours,
            event.gardener_name()
        );
        total_hours += event.hours;
    }

    for event in other_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        other_events += &format!(
            "{} at {} - {} hours by {}\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours,
            event.gardener_name()
        );
        total_hours += event.hours;
    }

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new("OG").icon_url(
            "https://liquipedia.net/commons/images/thumb/7/70/OG_RB_allmode.png/1200px-OG_RB_allmode.png"
        ))
        .title(format!("{} - {}", start.format("%B"), end.format("%B")))
        .fields(vec![
            ("Dota", dota_events, false),
            ("CS", cs_events, false),
            ("Other", other_events, false),
            ("Total Hours", total_hours.to_string(), false),
        ])
        .timestamp(Timestamp::now());
    Ok(embed)
}
