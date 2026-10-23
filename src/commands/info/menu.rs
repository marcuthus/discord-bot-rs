use poise::{
    CreateReply,
    serenity_prelude::{self as serenity, Mentionable},
};

use crate::client::{Context, Error};

const CATEGORY_LABELS: [(&str, &str); 4] = [
    ("info", "ℹ️ Informações"),
    ("group", "👥 Servidor"),
    ("media", "🖼️ Mídia"),
    ("admin", "🔐 Administração"),
];

struct Entry<'a> {
    category: &'a str,
    name: &'a str,
    description: &'a str,
}

fn group_by_category(entries: &[Entry]) -> Vec<(String, String)> {
    CATEGORY_LABELS
        .iter()
        .filter_map(|(category, label)| {
            let lines = entries
                .iter()
                .filter(|entry| entry.category == *category)
                .map(|entry| format!("• `/{}` - {}", entry.name, entry.description))
                .collect::<Vec<_>>();

            (!lines.is_empty()).then(|| ((*label).to_owned(), lines.join("\n")))
        })
        .collect()
}

/// Exibe o menu de comandos por categoria.
#[poise::command(slash_command, category = "info")]
pub async fn menu(ctx: Context<'_>) -> Result<(), Error> {
    let commands = &ctx.framework().options().commands;

    let entries = commands
        .iter()
        .map(|command| Entry {
            category: command.category.as_deref().unwrap_or("other"),
            name: &command.name,
            description: command.description.as_deref().unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    let bot = ctx.cache().current_user().clone();

    let embed = serenity::CreateEmbed::new()
        .author(serenity::CreateEmbedAuthor::new(&bot.name).icon_url(bot.face()))
        .title("📋 Menu")
        .description(format!(
            "Olá {}, estes são os meus comandos:",
            ctx.author().mention()
        ))
        .fields(
            group_by_category(&entries)
                .into_iter()
                .map(|(label, lines)| (label, lines, false)),
        )
        .colour(serenity::Colour::BLUE)
        .footer(
            serenity::CreateEmbedFooter::new("By Marcuth")
                .icon_url("https://avatars.githubusercontent.com/u/91915075?v=4"),
        );

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry<'a>(category: &'a str, name: &'a str) -> Entry<'a> {
        Entry {
            category,
            name,
            description: "descrição",
        }
    }

    #[test]
    fn groups_commands_following_the_category_order() {
        let groups = group_by_category(&[
            entry("admin", "set-status"),
            entry("info", "ping"),
            entry("info", "menu"),
        ]);

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].0, "ℹ️ Informações");
        assert_eq!(groups[0].1, "• `/ping` - descrição\n• `/menu` - descrição");
        assert_eq!(groups[1].0, "🔐 Administração");
    }

    #[test]
    fn skips_empty_and_unknown_categories() {
        let groups = group_by_category(&[entry("other", "x")]);

        assert!(groups.is_empty());
    }
}
