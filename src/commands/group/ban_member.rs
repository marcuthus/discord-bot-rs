use poise::serenity_prelude as serenity;

use crate::client::{Context, Error};

/// Bane um membro do servidor (requer permissão de banir).
#[poise::command(
    slash_command,
    rename = "ban-member",
    guild_only,
    category = "group",
    required_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS",
    default_member_permissions = "BAN_MEMBERS"
)]
pub async fn ban_member(
    ctx: Context<'_>,
    #[description = "Usuário que será banido."] user: serenity::User,
    #[description = "Motivo do banimento."] reason: Option<String>,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        return Ok(());
    };

    if user.id == ctx.author().id {
        ctx.say("Você não pode banir a si mesmo.").await?;
        return Ok(());
    }

    let reply = match guild_id
        .ban_with_reason(
            ctx.http(),
            user.id,
            0,
            reason.as_deref().unwrap_or_default(),
        )
        .await
    {
        Ok(()) => format!(
            "**{}** (`{}`) foi banido.{}",
            user.name,
            user.id,
            reason
                .map(|reason| format!(" Motivo: {reason}"))
                .unwrap_or_default()
        ),
        Err(error) => format!(
            "Não foi possível banir `{}`. O bot precisa de um cargo acima do alvo. ({error})",
            user.id
        ),
    };

    ctx.say(reply).await?;

    Ok(())
}
