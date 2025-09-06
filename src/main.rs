use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::{error::Error, fs};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&ctx.http, "pong!").await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = read_token(".env")?;

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    client.start().await?;
    Ok(())
}

fn read_token(env_path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(env_path)?;
    let token = content
        .split('=')
        .last()
        .ok_or("Could not read token")?;

    Ok(token.trim().to_string())
}
