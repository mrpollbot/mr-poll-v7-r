use poise;
use crate::{ Context, Error };

#[poise::command(prefix_command)]
pub async fn help(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Go help yourself").await?;
    Ok(())
}