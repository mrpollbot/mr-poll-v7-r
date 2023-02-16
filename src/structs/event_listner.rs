use crate::{ Data, Error };
use poise::serenity_prelude as serenity;

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