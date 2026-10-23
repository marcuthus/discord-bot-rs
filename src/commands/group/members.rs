use poise::{CreateReply, serenity_prelude as serenity};

use crate::client::{Context, Error};

const MEMBERS_PER_MESSAGE: usize = 25;
const MAX_MEMBERS: u64 = 100;

fn pages(lines: &[String]) -> Vec<String> {
    lines
        .chunks(MEMBERS_PER_MESSAGE)
        .map(|chunk| chunk.join("\n"))
        .collect()
}

/// Lista os membros do servidor e revela seus IDs.
#[poise::command(slash_command, guild_only, category = "group")]
pub async fn members(
    ctx: Context<'_>,
    #[description = "Quantidade de membros a listar (máximo 100, padrão 25)."]
    #[min = 1]
    #[max = 100]
    limit: Option<u64>,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        return Ok(());
    };

    ctx.defer().await?;

    let limit = limit.unwrap_or(MEMBERS_PER_MESSAGE as u64).min(MAX_MEMBERS);
    let members = guild_id.members(ctx.http(), Some(limit), None).await?;

    let lines = members
        .iter()
        .map(|member| {
            format!(
                "`{}` - {}",
                member.user.id,
                serenity::utils::content_safe(
                    ctx,
                    member.display_name(),
                    &serenity::ContentSafeOptions::default(),
                    &[]
                )
            )
        })
        .collect::<Vec<_>>();

    let title = format!(
        "👥 {} ({} membros)",
        ctx.guild()
            .map(|guild| guild.name.clone())
            .unwrap_or_default(),
        ctx.guild()
            .map(|guild| guild.member_count)
            .unwrap_or_default()
    );

    for (index, page) in pages(&lines).into_iter().enumerate() {
        let mut embed = serenity::CreateEmbed::new()
            .description(page)
            .colour(serenity::Colour::BLUE);

        if index == 0 {
            embed = embed.title(&title);
        }

        ctx.send(CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_members_into_pages() {
        let lines = (0..MEMBERS_PER_MESSAGE + 1)
            .map(|index| index.to_string())
            .collect::<Vec<_>>();

        let result = pages(&lines);

        assert_eq!(result.len(), 2);
        assert_eq!(result[1], MEMBERS_PER_MESSAGE.to_string());
    }

    #[test]
    fn has_no_pages_for_an_empty_list() {
        assert!(pages(&[]).is_empty());
    }
}
