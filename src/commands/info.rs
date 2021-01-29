use serenity::{
  framework::standard::{
      CommandResult,
      macros::command
  },
  model::prelude::*,
  prelude::*
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply_ping(ctx, "Pong!").await?;

    Ok(())
}
