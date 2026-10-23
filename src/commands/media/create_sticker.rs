use poise::serenity_prelude as serenity;

use crate::{
    client::{Context, Error},
    utils::sticker::{CropMode, StickerOptions, to_sticker},
};

const MAX_IMAGE_BYTES: u32 = 8 * 1024 * 1024;
const MAX_STICKER_BYTES: usize = 512 * 1024;

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum Crop {
    #[name = "true (preenche e corta)"]
    Fill,
    #[name = "false (imagem inteira)"]
    Fit,
    #[name = "dented (amassada)"]
    Dented,
}

impl From<Crop> for CropMode {
    fn from(crop: Crop) -> Self {
        match crop {
            Crop::Fill => Self::Fill,
            Crop::Fit => Self::Fit,
            Crop::Dented => Self::Dented,
        }
    }
}

/// Cria uma figurinha do servidor a partir de uma imagem (requer gerenciar expressões).
#[poise::command(
    slash_command,
    rename = "create-sticker",
    guild_only,
    category = "media",
    required_permissions = "MANAGE_GUILD_EXPRESSIONS",
    required_bot_permissions = "MANAGE_GUILD_EXPRESSIONS",
    default_member_permissions = "MANAGE_GUILD_EXPRESSIONS"
)]
pub async fn create_sticker(
    ctx: Context<'_>,
    #[description = "Imagem que virará figurinha."] image: serenity::Attachment,
    #[description = "Nome da figurinha (2 a 30 caracteres)."]
    #[min_length = 2]
    #[max_length = 30]
    name: String,
    #[description = "Tags de busca da figurinha (padrão: o nome)."] tags: Option<String>,
    #[description = "true (padrão): preenche e corta; false: imagem inteira; dented: amassada."]
    crop: Option<Crop>,
    #[description = "Centraliza o corte (padrão: sim). Não: alinha ao canto superior esquerdo."]
    center: Option<bool>,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        return Ok(());
    };

    let is_image = image
        .content_type
        .as_deref()
        .is_some_and(|content_type| content_type.starts_with("image/"));

    if !is_image || image.size > MAX_IMAGE_BYTES {
        ctx.say("Envie uma imagem de até 8 MB.").await?;
        return Ok(());
    }

    ctx.defer().await?;

    let options = StickerOptions {
        crop: crop.map(CropMode::from).unwrap_or_default(),
        center: center.unwrap_or(true),
    };

    let original = image.download().await?;
    let png = tokio::task::spawn_blocking(move || to_sticker(&original, options)).await??;

    if png.len() > MAX_STICKER_BYTES {
        ctx.say("A figurinha ficou maior que 512 KB. Tente uma imagem mais simples.")
            .await?;
        return Ok(());
    }

    let sticker = guild_id
        .create_sticker(
            ctx,
            serenity::CreateSticker::new(
                name.clone(),
                serenity::CreateAttachment::bytes(png, "sticker.png"),
            )
            .tags(tags.unwrap_or(name)),
        )
        .await?;

    ctx.say(format!(
        "Figurinha **{}** criada! (`{}`)",
        sticker.name, sticker.id
    ))
    .await?;

    Ok(())
}
