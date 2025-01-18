#![warn(clippy::pedantic)]

use dotenv::dotenv;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;

use poise::serenity_prelude::{self as serenity};

mod commands;
mod data;
use commands::*;

pub struct Data {
    db: Pool<Sqlite>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::ApplicationContext<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let db = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    env_logger::init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                ping(),
                signup::event(),
                signup::gardener(),
                signup::invoice(),
            ],
            on_error: |error| Box::pin(on_error(error)),
            pre_command: |ctx| {
                Box::pin(async move {
                    println!(
                        "Executing command {}... by {}",
                        ctx.command().qualified_name,
                        ctx.author().display_name()
                    );
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    println!(
                        "Executed command {} by {}!",
                        ctx.command().qualified_name,
                        ctx.author().display_name()
                    );
                })
            },

            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { db: db })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
