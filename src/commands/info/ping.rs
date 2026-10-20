use std::time::Instant;

use poise::{
    CreateReply,
    serenity_prelude::{self as serenity, Mentionable},
};

use crate::client::{Context, Error};

/// Envia o ping do bot.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let started = Instant::now();
    let handle = ctx.say("Calculando ping...").await?;
    let message_latency = started.elapsed().as_millis();
    let api_latency = ctx.ping().await.as_millis();

    let avatar_url = ctx.cache().current_user().face();

    let embed = serenity::CreateEmbed::new()
        .title("🏓 Pong!")
        .description(format!(
            "Olá {}, aqui estão os meus resultados:",
            ctx.author().mention()
        ))
        .field("📡 Latência da API", format!("`{api_latency}ms`"), true)
        .field(
            "✉️ Latência de Mensagem",
            format!("`{message_latency}ms`"),
            true,
        )
        .colour(serenity::Colour::BLUE)
        .thumbnail(avatar_url)
        .footer(
            serenity::CreateEmbedFooter::new("By Marcuth")
                .icon_url("https://avatars.githubusercontent.com/u/91915075?v=4"),
        )
        .timestamp(serenity::Timestamp::now());

    handle
        .edit(ctx, CreateReply::default().content("").embed(embed))
        .await?;

    Ok(())
}
