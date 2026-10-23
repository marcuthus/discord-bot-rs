use poise::serenity_prelude as serenity;

use crate::client::{Context, Error};

const SECONDS_PER_HOUR: u32 = 3600;

/// Cria um link de convite para este servidor.
#[poise::command(
    slash_command,
    guild_only,
    category = "group",
    required_permissions = "CREATE_INSTANT_INVITE",
    required_bot_permissions = "CREATE_INSTANT_INVITE",
    default_member_permissions = "CREATE_INSTANT_INVITE"
)]
pub async fn invite(
    ctx: Context<'_>,
    #[rename = "max-age-hours"]
    #[description = "Validade em horas (0 = não expira, padrão 24)."]
    #[min = 0]
    #[max = 168]
    max_age_hours: Option<u32>,
    #[rename = "max-uses"]
    #[description = "Número máximo de usos (0 = ilimitado, padrão 0)."]
    #[min = 0]
    #[max = 100]
    max_uses: Option<u8>,
) -> Result<(), Error> {
    let builder = serenity::CreateInvite::new()
        .max_age(max_age_hours.unwrap_or(24) * SECONDS_PER_HOUR)
        .max_uses(max_uses.unwrap_or(0))
        .unique(true);

    let reply = match ctx.channel_id().create_invite(ctx, builder).await {
        Ok(invite) => format!(
            "Link de convite de **{}**:\n{}",
            ctx.guild()
                .map(|guild| guild.name.clone())
                .unwrap_or_default(),
            invite.url()
        ),
        Err(_) => String::from(
            "Não foi possível criar o convite. O bot precisa da permissão de criar convites.",
        ),
    };

    ctx.say(reply).await?;

    Ok(())
}
