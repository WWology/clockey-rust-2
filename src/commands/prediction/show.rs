use poise::{
    serenity_prelude::{self as serenity, CreateEmbedAuthor, Mentionable, UserId},
    CreateReply,
};
use tabled::{
    builder::Builder,
    settings::{Alignment, Style},
};

use crate::{data, Context, Error};

#[allow(clippy::unused_async)]
#[poise::command(slash_command, subcommands("dota", "cs"))]
pub async fn show(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Show dota scoreboard or user score and rank
#[poise::command(slash_command)]
pub async fn dota(ctx: Context<'_>, member: Option<serenity::Member>) -> Result<(), Error> {
    ctx.defer().await?;

    if let Some(member) = member {
        if let Ok(score) =
            data::score::get_dota_score_for_id(&ctx.data().db, member.user.id.get()).await
        {
            ctx.reply(format!(
                "The Dota score prediction for {} is {} ranked at {}",
                member.mention(),
                score.score,
                score.rank
            ))
            .await?;
        } else {
            ctx.reply("That user isn't found on the scoreboard").await?;
        }
    } else {
        let scoreboard = data::score::show_dota_scoreboard(&ctx.data().db).await?;
        let total_page = scoreboard.len() / 10 + 1;
        let mut pages: Vec<String> = vec![];

        for page in 1..=total_page {
            let mut builder = Builder::new();
            builder.push_record(["Rank", "Name", "Score"]);

            let offset = (page - 1) * 10;
            let score_iter = scoreboard.iter().skip(offset).take(10);
            for score in score_iter {
                if let Ok(member) = ctx
                    .guild_id()
                    .ok_or("Failed to find guild")?
                    .member(&ctx, UserId::new(u64::try_from(score.id)?))
                    .await
                {
                    let name = truncate(member.display_name());
                    builder.push_record([score.rank.to_string(), name, score.score.to_string()]);
                } else if let Ok(user) = ctx
                    .http()
                    .get_user(UserId::new(u64::try_from(score.id)?))
                    .await
                {
                    let name = truncate(user.display_name());
                    builder.push_record([score.rank.to_string(), name, score.score.to_string()]);
                } else {
                    let name = format!("{}...", &score.id.to_string()[0..9]);
                    builder.push_record([score.rank.to_string(), name, score.score.to_string()]);
                }
            }
            let table = builder
                .build()
                .with(Style::markdown().remove_left().remove_right())
                .with(Alignment::center_vertical())
                .with(Alignment::center())
                .to_string();
            pages.push(table);
        }

        paginate(ctx, pages, "Dota Prediction Leaderboard").await?;
    }
    Ok(())
}

/// Show cs scoreboard or user score and rank
#[poise::command(slash_command)]
pub async fn cs(ctx: Context<'_>, member: Option<serenity::Member>) -> Result<(), Error> {
    ctx.defer().await?;

    if let Some(member) = member {
        if let Ok(score) =
            data::score::get_cs_score_for_id(&ctx.data().db, member.user.id.get()).await
        {
            ctx.reply(format!(
                "The CS score prediction for {} is {} ranked at {}",
                member.mention(),
                score.score,
                score.rank
            ))
            .await?;
        } else {
            ctx.reply("That user isn't found on the scoreboard").await?;
        }
    } else {
        let scoreboard = data::score::show_cs_scoreboard(&ctx.data().db).await?;
        let total_page = scoreboard.len() / 10 + 1;
        let mut pages: Vec<String> = vec![];

        for page in 1..=total_page {
            let mut builder = Builder::new();
            builder.push_record(["Rank", "Name", "Score"]);

            let offset = (page - 1) * 10;
            let score_iter = scoreboard.iter().skip(offset).take(10);
            for score in score_iter {
                if let Ok(member) = ctx
                    .guild_id()
                    .ok_or("Failed to find guild")?
                    .member(&ctx, UserId::new(u64::try_from(score.id)?))
                    .await
                {
                    let name = truncate(member.display_name());
                    builder.push_record([score.rank.to_string(), name, score.score.to_string()]);
                } else if let Ok(user) = ctx
                    .http()
                    .get_user(UserId::new(u64::try_from(score.id)?))
                    .await
                {
                    let name = truncate(user.display_name());
                    builder.push_record([score.rank.to_string(), name, score.score.to_string()]);
                } else {
                    let name = format!("{}...", &score.id.to_string()[0..9]);
                    builder.push_record([score.rank.to_string(), name, score.score.to_string()]);
                }
            }
            let table = builder
                .build()
                .with(Style::markdown().remove_left().remove_right())
                .with(Alignment::center_vertical())
                .with(Alignment::center())
                .to_string();
            pages.push(table);
        }

        paginate(ctx, pages, "CS Prediction Leaderboard").await?;
    }
    Ok(())
}

fn truncate(name: &str) -> String {
    if name.len() > 12 {
        return format!("{}...", &name[0..9]);
    }
    name.to_string()
}

pub async fn paginate(
    ctx: Context<'_>,
    pages: Vec<String>,
    title: &str,
) -> Result<(), serenity::Error> {
    // Define some unique identifiers for the navigation buttons
    let ctx_id = ctx.id();
    let prev_button_id = format!("{ctx_id}prev");
    let next_button_id = format!("{ctx_id}next");

    // Send the embed with the first page as content
    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&prev_button_id).emoji('◀'),
            serenity::CreateButton::new(&next_button_id).emoji('▶'),
        ]);

        CreateReply::default()
            .embed(
                serenity::CreateEmbed::new().title(title)
                    .author(
                        CreateEmbedAuthor::new("OG").icon_url(
                            "https://liquipedia.net/commons/images/thumb/7/70/OG_RB_allmode.png/1200px-OG_RB_allmode.png"
                        ),
                    )
                    .field("",format!("```{}```", pages[0].clone()), true)
                    .field("Can't see yourself?", "Use /show @yourself to see where you stand", false),
            )
            .components(vec![components])
    };

    ctx.send(reply).await?;

    // Loop through incoming interactions with the navigation buttons
    let mut current_page = 0;
    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        // Timeout when no navigation button has been pressed for 24 hours
        .timeout(std::time::Duration::from_secs(600))
        .await
    {
        // Depending on which button was pressed, go to next or previous page
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            // This is an unrelated button interaction
            continue;
        }

        // Update the message with the new page contents
        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new().embed(
                        serenity::CreateEmbed::new().title(title)
                            .author(CreateEmbedAuthor::new("OG").icon_url(
                                "https://liquipedia.net/commons/images/thumb/7/70/OG_RB_allmode.png/1200px-OG_RB_allmode.png"
                            ))
                            .field("", format!("```{}```", pages[current_page].clone()), true)
                            .field("Can't see yourself?", "Use /show @yourself to see where you stand", false),

                    ),
                ),
            )
            .await?;
    }

    Ok(())
}
