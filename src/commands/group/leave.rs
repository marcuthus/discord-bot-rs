use crate::client::{Context, Error};

/// Faz o bot sair do servidor (requer gerenciar servidor).
#[poise::command(
    slash_command,
    guild_only,
    category = "group",
    required_permissions = "MANAGE_GUILD",
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        return Ok(());
    };

    ctx.say("Até mais! 👋").await?;
    guild_id.leave(ctx.http()).await?;

    Ok(())
}
