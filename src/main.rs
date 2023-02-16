use poise::serenity_prelude as serenity;
use serenity::{
    Context,
    Error
};
use dotenv;

pub struct Data {

}

#[poise::command(prefix_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Text to say"]
    msg: String,
) -> Result<(), Error> {
    ctx.say(msg).await?;
    Ok(())
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

#[tokio::main]
async fn main() {
    dotenv::dotenv();

    let options = poise::FrameworkOptions {
        commands: vec![
            say()
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("dev".into()),
            ignore_bots: true,
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
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
        serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_MEMBERS,
    )
    .run()
    .await
    .unwrap();
}