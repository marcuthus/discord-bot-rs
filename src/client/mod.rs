mod data;
mod error;

use std::sync::Arc;

use poise::serenity_prelude as serenity;
use tracing::info;

use crate::{commands, config::Config, events};

pub use data::{Context, Data, Error};

pub async fn start(config: Config) -> anyhow::Result<()> {
    info!(target: "client", "Starting...");

    let token = config.token.clone();
    let config = Arc::new(config);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::all(),
            on_error: |error| Box::pin(error::handle(error)),
            event_handler: |framework, event| Box::pin(events::handle(framework, event)),
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!(target: "client", "All slash commands synchronized with Discord!");

                Ok(Data { config })
            })
        })
        .build();

    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
