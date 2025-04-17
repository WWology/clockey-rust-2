#![warn(clippy::pedantic)]

use poise::{
    serenity_prelude::{
        ChunkGuildFilter, ClientBuilder, EmojiId, FullEvent, GatewayIntents, ReactionType, RoleId,
    },
    FrameworkContext,
};
use sqlx::{Pool, Sqlite, SqlitePool};

mod commands;
mod data;
use commands::{next, ping, prediction, signup};

pub struct Data {
    db: Pool<Sqlite>,
    config: Config,
}

pub struct Config {
    signup_emoji: ReactionType,
    processed_emoji: ReactionType,
    dota_channel: u64,
    cs_channel: u64,
    rivals_channel: u64,
    mlbb_channel: u64,
    hok_channel: u64,
    stage_channel: u64,
    dota_oracle_role: RoleId,
    cs2_awpacle_role: RoleId,
    rivals_avengers_role: RoleId,
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
                println!("Error while handling error: {e:?}");
            }
        }
    }
}

#[allow(clippy::unused_async)]
async fn event_handler(
    framework: FrameworkContext<'_, Data, Error>,
    event: &FullEvent,
) -> Result<(), Error> {
    let ctx = framework.serenity_context;
    match event {
        FullEvent::CacheReady { guilds } => {
            for guild_id in guilds {
                ctx.shard
                    .chunk_guild(*guild_id, None, true, ChunkGuildFilter::None, None);
            }
        }
        FullEvent::GuildMembersChunk { chunk } => {
            println!(
                "Chunked guild {} with member count {}",
                chunk.guild_id, chunk.chunk_count
            );
        }
        _ => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env!("DISCORD_TOKEN");
    let intents = GatewayIntents::all();

    let db = SqlitePool::connect(env!("DATABASE_URL"))
        .await
        .expect("Unable to connect to Database");

    let config = init_config();

    tracing_subscriber::fmt()
        .with_target(true)
        .with_line_number(true)
        .init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // General purpose command
                ping(),
                next(),
                // Signup related command
                signup::cancel(),
                signup::edit(),
                signup::event(),
                signup::gardener(),
                signup::invoice(),
                signup::manual(),
                signup::report(),
                // Prediction related command
                prediction::csadd(),
                prediction::csbo(),
                prediction::deletecs(),
                prediction::deletedota(),
                prediction::deleteextra(),
                prediction::deletehok(),
                prediction::deletemlbb(),
                prediction::deleterivals(),
                prediction::dotaadd(),
                prediction::dotabo(),
                prediction::extrabo(),
                prediction::hokadd(),
                prediction::hokbo(),
                prediction::mlbbadd(),
                prediction::mlbbbo(),
                prediction::rivalsadd(),
                prediction::rivalsbo(),
                prediction::show(),
                prediction::winners(),
            ],
            on_error: |error| Box::pin(on_error(error)),
            event_handler: |framework, event| Box::pin(event_handler(framework, event)),
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
                Ok(Data { db, config })
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client
        .expect("Unable to build client")
        .start()
        .await
        .expect("Unable to start client");
}

fn init_config() -> Config {
    // Emojis initialisation
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

    // Channels initialisation
    let dota_channel = if cfg!(debug_assertions) {
        738_607_620_566_286_398
    } else {
        738_009_797_932_351_519
    };
    let cs_channel = if cfg!(debug_assertions) {
        738_607_620_566_286_398
    } else {
        746_618_267_434_614_804
    };
    let rivals_channel = if cfg!(debug_assertions) {
        738_607_620_566_286_398
    } else {
        1_344_498_244_830_498_836
    };
    let mlbb_channel = if cfg!(debug_assertions) {
        738_607_620_566_286_398
    } else {
        1_350_253_132_168_429_801
    };
    let hok_channel = if cfg!(debug_assertions) {
        738_607_620_566_286_398
    } else {
        1_344_676_973_313_658_920
    };
    let stage_channel = if cfg!(debug_assertions) {
        991_620_472_544_440_454
    } else {
        1_186_593_338_300_842_025
    };

    // Prediction winners role
    let dota_oracle_role = RoleId::new(729_106_634_437_296_148);
    let cs2_awpacle_role = RoleId::new(729_106_753_085_636_688);
    let rivals_avengers_role = RoleId::new(1_347_178_660_192_583_731);
    Config {
        signup_emoji,
        processed_emoji,
        dota_channel,
        cs_channel,
        rivals_channel,
        mlbb_channel,
        hok_channel,
        stage_channel,
        dota_oracle_role,
        cs2_awpacle_role,
        rivals_avengers_role,
    }
}
