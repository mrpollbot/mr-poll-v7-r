use poise::serenity_prelude::{self as serenity, UserId};
use dotenv::dotenv;
mod commands;
mod structs;
use crate::{ 
    commands::general, 
    structs::error_handling,
    structs::event_listner::listener, 
};
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const OWNERS_IDS: Vec<UserId> = vec![579466943170609153, 402888568579686401];
pub struct Data {
    
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
        on_error: |error| Box::pin(error_handling::on_error(error)),
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
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.command().owners_only {
                    if OWNERS_IDS.contains(&ctx.author().id) {
                        return Ok(true)
                    }
                    return Ok(false)
                }
                Ok(true)
            })
        }),
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