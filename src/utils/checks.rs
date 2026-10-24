use crate::client::{Context, Error};

pub fn is_bot_owner_id(id: u64, admin_id: &str) -> bool {
    id.to_string() == admin_id
}

pub async fn is_bot_owner(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(is_bot_owner_id(
        ctx.author().id.get(),
        &ctx.data().config.admin_id,
    ))
}

/// O bot bane/modera com as próprias permissões, então a hierarquia do invocador precisa ser checada
/// manualmente: só pode agir sobre alguém com cargo mais alto que o alvo (o dono do servidor e o dono
/// do bot são isentos) e nunca sobre o dono do servidor.
pub fn can_moderate(
    author_id: u64,
    target_id: u64,
    guild_owner_id: u64,
    author_top_role: u16,
    target_top_role: u16,
    author_is_bot_owner: bool,
) -> bool {
    if target_id == guild_owner_id {
        return false;
    }

    author_is_bot_owner || author_id == guild_owner_id || author_top_role > target_top_role
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moderation_respects_role_hierarchy() {
        assert!(can_moderate(1, 2, 9, 5, 3, false));
        assert!(!can_moderate(1, 2, 9, 3, 3, false));
        assert!(!can_moderate(1, 2, 9, 2, 3, false));
        assert!(can_moderate(9, 2, 9, 0, 10, false));
        assert!(can_moderate(1, 2, 9, 0, 10, true));
        assert!(!can_moderate(1, 9, 9, 10, 0, true));
    }

    #[test]
    fn matches_only_the_configured_admin() {
        assert!(is_bot_owner_id(123, "123"));
        assert!(!is_bot_owner_id(456, "123"));
        assert!(!is_bot_owner_id(123, ""));
    }
}
