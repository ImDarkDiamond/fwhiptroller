use futures::StreamExt;
use tracing::{error, info};
use std::{env::var, error::Error};

use twilight_http::Client;
use twilight_model::id::RoleId;
use twilight_gateway::{Event, Intents, Shard};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv::dotenv().ok();

    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = var("TOKEN")?;
    let troll_role = var("TROLL_ROLE")?.parse::<u64>()?;

    let http_client = Client::new(&token);

    let intents = Intents::GUILD_MEMBERS;
    let mut shard = Shard::new(&token, intents);
    let mut events = shard.events();

    shard.start().await?;

    while let Some(event) = events.next().await {
        match event {
            Event::MemberUpdate(member) => {
                for role_id in member.roles {
                    if role_id == RoleId(troll_role) {
                        info!("Member {}#{} has the trolling role. Kicking now...", member.user.name, member.user.discriminator);
                        if let Err(e) = http_client.remove_guild_member(member.guild_id, member.user.id).await {
                            error!("Error kicking member: {}", e);
                        } else {
                            info!("Kicked a troll: {}#{}", member.user.name, member.user.discriminator);
                        }
                        break
                    }
                }
            },

            Event::ShardConnected(_) => {
                info!("Bot is ready");
            }
            Event::ShardDisconnected(_) => {
                info!("Bot disconnected. Reconnecting now...");
            }
            _ => {}
        }
    }

    Ok(())
}