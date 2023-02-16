use poise::serenity_prelude as serenity;
use dotenv::dotenv;
mod commands;
use crate::{ commands::general };
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

pub async fn listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { .. } => {
            ctx.set_activity(serenity::Activity::watching("for /poll")).await;
            println!("Bot is Online");
        }
        _ => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().expect("Missing .env file.");

    let options = poise::FrameworkOptions {
        commands: vec![
            general::help()
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("dev".into()),
            ignore_bots: true,
            mention_as_prefix: true,
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        event_handler: | ctx, event, _framework, data| {
            Box::pin(async move {
                println!("Got an event in event handler: {:?}", event.name());
                listener(ctx, event, data).await?;
                Ok(())
            })
        },
        allowed_mentions: None,
        ..Default::default()
    };

    poise::Framework::builder()
    .token(
        dotenv::var("BOT_TOKEN")
        .expect("Missing key `BOT_TOKEN` in env. Please check the readme.")
    )
    .setup(move |_ctx, _ready, _framework| {
        Box::pin(async move {
            Ok(Data {
                
            })
        })
    })
    .options(options)
    .intents(
        serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::GUILD_MEMBERS | serenity::GatewayIntents::GUILDS
    )
    .run_autosharded()
    .await
    .unwrap();
}