use serenity::{all::CreateAttachment, async_trait};
use serenity::model::channel::Message;
use serenity::http::Http;
use serenity::model::id::ChannelId;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use serenity::builder::{ CreateMessage};

use tokio::fs::File;
use std::{error::Error, fs, path::Path, sync::Arc};

mod bird_reader;

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


async fn send_to_discord(http: &Http, channel: ChannelId, content: &str) {
    let _ = channel.say(http, content).await;
}



pub async fn send_image(
    http: &Http,
    channel: ChannelId,
    image_path: &str,
    message: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !Path::new(image_path).exists() {
        eprintln!("file not found: {}", image_path);
        return Ok(());
    }

    let f = File::open(image_path).await?;

    let attachment = CreateAttachment::file(&f, Path::new(image_path)
        .file_name()
        .and_then(|os| os.to_str())
        .ok_or("invalid filename")?
    ).await?;

    let builder = CreateMessage::new()
        .content(message)
        .add_file(attachment);

    channel.send_message(http, builder).await?;

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = read_token(".env")?;

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    let _ = client.start();



    let http = client.http;
    bird_reader::live_bird_feed(http, ChannelId::new(1224514100210569327)).await;



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
