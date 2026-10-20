use std::env;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("a variável de ambiente `{0}` não foi definida")]
    Missing(&'static str),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
    pub admin_id: String,
    // Disponíveis para quem for estender o template (ex.: webhooks de interação).
    #[allow(dead_code)]
    pub application_id: String,
    #[allow(dead_code)]
    pub public_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            token: required("DISCORD_BOT_TOKEN")?,
            admin_id: required("DISCORD_ADMIN_ID")?,
            application_id: required("DISCORD_APPLICATION_ID")?,
            public_key: required("DISCORD_PUBLIC_KEY")?,
        })
    }
}

fn required(key: &'static str) -> Result<String, ConfigError> {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => Ok(value),
        _ => Err(ConfigError::Missing(key)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_fails_when_variable_is_missing() {
        let error = required("DISCORD_BOT_RS_UNSET_VARIABLE").unwrap_err();

        assert_eq!(
            error.to_string(),
            "a variável de ambiente `DISCORD_BOT_RS_UNSET_VARIABLE` não foi definida"
        );
    }
}
