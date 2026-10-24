use poise::{CreateReply, serenity_prelude as serenity};

use crate::{
    client::{Context, Error},
    utils::checks::{can_moderate, is_bot_owner_id},
};

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

    let author_member = ctx.author_member().await.map(|member| member.into_owned());
    let target_member = guild_id.member(ctx.http(), user.id).await.ok();

    // Posição do maior cargo de cada um (sem segurar a referência do cache através de um await)
    let (owner_id, author_top_role, target_top_role) = {
        let Some(guild) = ctx.guild() else {
            return Ok(());
        };

        let top_role = |member: &Option<serenity::Member>| {
            member
                .as_ref()
                .map(|member| {
                    member
                        .roles
                        .iter()
                        .filter_map(|role_id| guild.roles.get(role_id))
                        .map(|role| role.position)
                        .max()
                        .unwrap_or(0)
                })
                .unwrap_or(0)
        };

        (
            guild.owner_id.get(),
            top_role(&author_member),
            top_role(&target_member),
        )
    };

    if !can_moderate(
        ctx.author().id.get(),
        user.id.get(),
        owner_id,
        author_top_role,
        target_top_role,
        is_bot_owner_id(ctx.author().id.get(), &ctx.data().config.admin_id),
    ) {
        ctx.send(
            CreateReply::default()
                .content("Você não pode banir este usuário (cargo igual/superior ao seu ou dono do servidor).")
                .ephemeral(true),
        )
        .await?;
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
