mod ready;

use poise::serenity_prelude as serenity;

use crate::client::{Data, Error};

pub async fn handle(
    framework: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            ready::run(framework.serenity_context, data_about_bot).await
        }
        _ => Ok(()),
    }
}
