use crate::ShardManagerContainer;
use serenity::{
  framework::standard::{
      CommandResult,
      macros::command
  },
  model::{
      prelude::*
  },
  prelude::*
};
use colour::{
    magenta_ln
};

#[command]
#[owners_only]
/// Closes the bot in the same fashion as ctrl+c. Restarts if under systemcd.
/// **Example usage:** `&rust quit`
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let guild_name = if msg.guild_id.is_some() {
        msg.guild_id.unwrap().name(ctx).await
    } else {
        Some("DMs".to_string())
    };

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply_ping(ctx, "Shutting down...").await?;
        magenta_ln!("Shutting down from command ran by {:?}#{:?} in {:?}", msg.author.name,
                 msg.author.discriminator, guild_name.unwrap());
        manager.lock().await.shutdown_all().await;
    } else {
            msg.reply_ping(ctx, "There was a problem getting the shard manager").await?;

        return Ok(());
    }

    Ok(())
}