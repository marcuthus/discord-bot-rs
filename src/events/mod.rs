mod ready;

use poise::serenity_prelude as serenity;

use crate::client::{Data, Error};

/// Despacha cada evento do Discord para o seu módulo em `src/events/`.
///
/// Para tratar um novo evento, crie `src/events/<evento>.rs` e adicione um
/// braço ao `match` abaixo.
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
