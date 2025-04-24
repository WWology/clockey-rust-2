use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use poise::{
    serenity_prelude::{futures::future, CreateEmbed, CreateEmbedAuthor, Timestamp},
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
    #[description = "Should report be separated by gardener / game"] report_option: ReportOption,
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

    match report_option {
        ReportOption::Gardener => {
            let report = generate_report_per_gardener(&ctx, start, end).await?;
            ctx.send(CreateReply::new().embed(report)).await?;
        }
        ReportOption::Game => {
            let report = generate_report_per_game(&ctx, start, end).await?;
            ctx.send(CreateReply::new().embed(report)).await?;
        }
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
async fn generate_report_per_gardener(
    ctx: &Context<'_>,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<CreateEmbed, Error> {
    let (nik_invoice, kit_invoice, ww_invoice, bonteng_invoice, sam_invoice) = future::try_join5(
        Event::get_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            293_360_731_867_316_225,
        ),
        Event::get_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            204_923_365_205_475_329,
        ),
        Event::get_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            754_724_309_276_164_159,
        ),
        Event::get_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            172_360_818_715_918_337,
        ),
        Event::get_events_for_id(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
            332_438_787_588_227_072,
        ),
    )
    .await?;

    let (mut nik_events, mut kit_events, mut ww_events, mut bonteng_events, mut sam_events): (
        String,
        String,
        String,
        String,
        String,
    ) = (
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    );
    let mut total_hours: i64 = 0;

    for event in nik_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        nik_events += format!(
            "{} at {} - {} hours\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours
        )
        .as_str();
        total_hours += event.hours;
    }

    for event in kit_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        kit_events += &format!(
            "{} at {} - {} hours\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours
        );
        total_hours += event.hours;
    }

    for event in ww_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        ww_events += &format!(
            "{} at {} - {} hours\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours
        );
        total_hours += event.hours;
    }

    for event in bonteng_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        bonteng_events += &format!(
            "{} at {} - {} hours\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours
        );
        total_hours += event.hours;
    }

    for event in sam_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        sam_events += &format!(
            "{} at {} - {} hours\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours
        );
        total_hours += event.hours;
    }

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new("OG").icon_url(
            "https://liquipedia.net/commons/images/thumb/7/70/OG_RB_allmode.png/1200px-OG_RB_allmode.png"
        ))
        .title(format!("{} - {}", start.format("%B"), end.format("%B")))
        .fields(vec![
            ("Nik", nik_events, false),
            ("Kit", kit_events, false),
            ("WW", ww_events, false),
            ("Bonteng", bonteng_events, false),
            ("Sam", sam_events, false),
            ("Total Hours", total_hours.to_string(), false),
        ])
        .timestamp(Timestamp::now());
    Ok(embed)
}

#[allow(clippy::too_many_lines)]
async fn generate_report_per_game(
    ctx: &Context<'_>,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<CreateEmbed, Error> {
    let (dota_invoice, cs_invoice, rivals_invoice) = future::try_join3(
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
        Event::get_rivals_events(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
        ),
    )
    .await?;

    let (mlbb_invoice, hok_invoice, other_invoice) = future::try_join3(
        Event::get_mlbb_events(
            &ctx.data().db,
            start.and_utc().timestamp(),
            end.and_utc().timestamp(),
        ),
        Event::get_hok_events(
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

    for event in rivals_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        rivals_events += &format!(
            "{} at {} - {} hours by {}\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours,
            event.gardener_name()
        );
        total_hours += event.hours;
    }

    for event in mlbb_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        mlbb_events += &format!(
            "{} at {} - {} hours by {}\n",
            event.name,
            time.format("%e %b, %Y"),
            event.hours,
            event.gardener_name()
        );
        total_hours += event.hours;
    }

    for event in hok_invoice {
        let time = DateTime::from_timestamp(event.time, 0).ok_or("Invalid Timestamp")?;
        hok_events += &format!(
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
            ("Rivals", rivals_events, false),
            ("MLBB", mlbb_events, false),
            ("HoK", hok_events, false),
            ("Other", other_events, false),
            ("Total Hours", total_hours.to_string(), false),
        ])
        .timestamp(Timestamp::now());
    Ok(embed)
}

#[derive(poise::ChoiceParameter)]
enum ReportOption {
    #[name = "Per Gardener"]
    Gardener,

    #[name = "Per Game"]
    Game,
}
