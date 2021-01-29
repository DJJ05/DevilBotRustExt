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
/// Returns bot response time (time between command start and finish).
/// **Example usage:** `&rust ping`
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = Instant::now();
    let mut reply = msg.reply_ping(ctx, "Pong!").await?;
    let duration = start.elapsed();
    reply.edit(ctx, |c|
        c.content(format!("Response Time (Latency): `{:?}`", duration))).await?;

    Ok(())
}

#[command]
/// Returns full command list, command help, or group help.
/// **Example usage:** `&rust help ping`
async fn help(_: &Context, _: &Message) -> CommandResult {
    Ok(())
}
