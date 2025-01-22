#![warn(clippy::pedantic)]

use poise::serenity_prelude::{self as serenity, EmojiId, ReactionType};
use sqlx::{Pool, Sqlite, SqlitePool};

mod commands;
mod data;
use commands::{ping, signup};

pub struct Data {
    db: Pool<Sqlite>,
    signup_emoji: ReactionType,
    processed_emoji: ReactionType,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::ApplicationContext<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {error:?}"),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {e}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env!("DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let db = SqlitePool::connect(env!("DATABASE_URL"))
        .await
        .expect("Unable to connect to Database");

    let signup_emoji = if cfg!(debug_assertions) {
        ReactionType::Custom {
            animated: false,
            id: EmojiId::new(951_843_834_554_376_262),
            name: Some(String::from("ruggahPain")),
        }
    } else {
        ReactionType::Custom {
            animated: false,
            id: EmojiId::new(730_890_894_814_740_541),
            name: Some(String::from("OGpeepoYes")),
        }
    };
    let processed_emoji = if cfg!(debug_assertions) {
        ReactionType::Custom {
            animated: false,
            id: EmojiId::new(1_329_032_244_580_323_349),
            name: Some(String::from("khezuBrain")),
        }
    } else {
        ReactionType::Custom {
            animated: false,
            id: EmojiId::new(787_697_278_190_223_370),
            name: Some(String::from("OGwecoo")),
        }
    };

    tracing_subscriber::fmt::init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                ping(),
                signup::event(),
                signup::gardener(),
                signup::invoice(),
                signup::manual(),
                signup::edit(),
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
                Ok(Data {
                    db,
                    signup_emoji,
                    processed_emoji,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client
        .expect("Unable to build client")
        .start()
        .await
        .expect("Unable to start client");
}
