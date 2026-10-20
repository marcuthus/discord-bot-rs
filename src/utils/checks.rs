use crate::client::{Context, Error};

/// Verifica se o ID informado pertence ao dono (admin) configurado do bot.
pub fn is_bot_owner_id(id: u64, admin_id: &str) -> bool {
    id.to_string() == admin_id
}

/// Check do poise: libera o comando apenas para o dono do bot.
pub async fn is_bot_owner(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(is_bot_owner_id(
        ctx.author().id.get(),
        &ctx.data().config.admin_id,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_only_the_configured_admin() {
        assert!(is_bot_owner_id(123, "123"));
        assert!(!is_bot_owner_id(456, "123"));
        assert!(!is_bot_owner_id(123, ""));
    }
}
