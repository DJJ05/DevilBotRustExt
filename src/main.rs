use std::{
    collections::HashSet,
    sync::Arc
};
use serenity::{
    async_trait,
    http::Http,
    prelude::*,
    client::{
        bridge::gateway::ShardManager,
        Client,
        Context,
        EventHandler
    },
    model::{
        gateway::Ready,
        channel::Message,
        prelude::{
            UserId
        }
    },
    framework::standard::{
        StandardFramework,
        help_commands,
        Args,
        HelpOptions,
        CommandGroup,
        CommandResult,
        macros::{
            group,
            help
        }
    },
    utils::Colour
};
use colour::{
    green_ln,
    red_ln
};
mod config;
mod commands;
use commands::{
    info::*,
    owner::*,
    utility::*
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let eligible_guilds: Vec<u64> = vec![696343847210319924, 745942562648621109];
        let mention_strings: Vec<&str> = vec!["<@!720229743974285312>", "<@720229743974285312>"];
        let mut mentions_me: bool = false;

        if let Some(guild_id) = msg.guild_id {
            if eligible_guilds.contains(&guild_id.0) {
                let content: &str = &msg.content.to_lowercase();
                if mention_strings.contains(&content) {
                    mentions_me = true;
                }
            }
        }

        if mentions_me {
            if let Err(why) = msg.reply_ping(ctx,
            "The DevilBot Rust branch is available in this guild. Use `&rust help` \
            to see the faster, alternate commands available.").await {
            red_ln!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        green_ln!("Client connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {

    let token = config::token();

    let http = Http::new_with_token(token);
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Error getting current application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .with_whitespace(true)
            .prefix("&rust "))
            .help(&DEVILBOT_HELP)
            .group(&OWNER_GROUP)
            .group(&INFO_GROUP)
            .group(&UTILITY_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        red_ln!("Error running client: {:?}", why);
    }
}

#[help]
async fn devilbot_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let mut ho = help_options.clone();
    ho.embed_success_colour = Colour::from(0xb7410e);
    let _ = help_commands::with_embeds(context, msg, args, &ho, groups, owners).await;
    Ok(())
}

#[group]
#[commands(ping, help)]
/// Group of commands dedicated to information gathering.
struct Info;

#[group]
#[commands(quit)]
/// Group of commands dedicated to owner-only operations.
struct Owner;

#[group]
#[commands(wiki)]
/// Group of commands dedicated to utility and efficiency operations.
struct Utility;
