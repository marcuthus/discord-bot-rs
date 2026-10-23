use crate::client::{Context, Error};

/// Mostra o ID do servidor e do canal atuais.
#[poise::command(slash_command, rename = "server-id", guild_only, category = "info")]
pub async fn server_id(ctx: Context<'_>) -> Result<(), Error> {
    let guild_name = ctx
        .guild()
        .map(|guild| guild.name.clone())
        .unwrap_or_default();

    ctx.say(format!(
        "Servidor **{guild_name}**: `{}`\nCanal: `{}`",
        ctx.guild_id().map(|id| id.get()).unwrap_or_default(),
        ctx.channel_id()
    ))
    .await?;

    Ok(())
}
