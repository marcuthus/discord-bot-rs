use poise::serenity_prelude as serenity;

use crate::client::{Context, Error};

/// Mostra o seu ID ou o de outro usuário.
#[poise::command(slash_command, rename = "user-id", category = "info")]
pub async fn user_id(
    ctx: Context<'_>,
    #[description = "Usuário que terá o ID revelado (padrão: você)."] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());

    ctx.say(format!("O ID de **{}** é `{}`", user.name, user.id))
        .await?;

    Ok(())
}
