use poise::{CreateReply, serenity_prelude as serenity};

use crate::{
    client::{Context, Error},
    utils::checks::is_bot_owner,
};

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum Status {
    #[name = "Online"]
    Online,
    #[name = "Ausente"]
    Idle,
    #[name = "Não perturbar"]
    DoNotDisturb,
    #[name = "Invisível"]
    Invisible,
}

impl Status {
    fn label(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Idle => "idle",
            Self::DoNotDisturb => "dnd",
            Self::Invisible => "invisible",
        }
    }

    fn emoji(self) -> &'static str {
        match self {
            Self::Online => "🟢",
            Self::Idle => "🌙",
            Self::DoNotDisturb => "🔴",
            Self::Invisible => "⚫",
        }
    }

    fn colour(self) -> serenity::Colour {
        match self {
            Self::Online => serenity::Colour::DARK_GREEN,
            Self::Idle => serenity::Colour::PURPLE,
            Self::DoNotDisturb => serenity::Colour::RED,
            Self::Invisible => serenity::Colour::DARK_GREY,
        }
    }

    fn online_status(self) -> serenity::OnlineStatus {
        match self {
            Self::Online => serenity::OnlineStatus::Online,
            Self::Idle => serenity::OnlineStatus::Idle,
            Self::DoNotDisturb => serenity::OnlineStatus::DoNotDisturb,
            Self::Invisible => serenity::OnlineStatus::Invisible,
        }
    }
}

#[poise::command(slash_command, rename = "set-status", check = "is_bot_owner")]
pub async fn set_status(
    ctx: Context<'_>,
    #[rename = "new-status"]
    #[description = "Digite o novo status que você deseja que seja aplicado."]
    new_status: Status,
    #[rename = "new-description"]
    #[description = "Nova descrição a ser definida."]
    new_description: String,
) -> Result<(), Error> {
    let (bot_name, bot_avatar) = {
        let bot = ctx.cache().current_user();
        (bot.name.to_string(), bot.face())
    };

    let embed = serenity::CreateEmbed::new()
        .author(serenity::CreateEmbedAuthor::new(bot_name).icon_url(bot_avatar))
        .title("Novo status e descrição aplicados!")
        .description(format!(
            "⎜\n⎜➦ {} Novo Status: `{}`\n⎜\n⎜➦ 📋 Nova Descrição: `{}`\n⎜",
            new_status.emoji(),
            new_status.label(),
            new_description
        ))
        .colour(new_status.colour());

    ctx.send(CreateReply::default().embed(embed).ephemeral(true))
        .await?;

    ctx.serenity_context().set_presence(
        Some(serenity::ActivityData::custom(new_description)),
        new_status.online_status(),
    );

    Ok(())
}
