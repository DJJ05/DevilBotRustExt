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
        channel::Message
    },
    framework::standard::{
        StandardFramework,
        macros::{
            group
        }
    }
};
use colour::{
    green_ln,
    red_ln
};
mod config;
mod commands;
use commands::{
    info::*,
    owner::*
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase() == "devilbot is the best" {
            if let Err(why) = msg.reply_ping(ctx, "Yes he is").await {
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
        .group(&OWNER_GROUP)
        .group(&INFO_GROUP);

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

#[group]
#[commands(ping)]
struct Info;

#[group]
#[commands(quit)]
struct Owner;
