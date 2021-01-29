use std::time::{
    Instant
};
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
    let start = Instant::now();
    let mut reply = msg.reply_ping(ctx, "Pong!").await?;
    let duration = start.elapsed();
    reply.edit(ctx, |c|
        c.content(format!("Response Time (Latency): `{:?}`", duration))).await?;

    Ok(())
}
