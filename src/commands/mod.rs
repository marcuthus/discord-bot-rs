mod admin;
mod info;

use crate::client::{Data, Error};

/// Agrega os comandos de todas as categorias.
///
/// Para adicionar uma categoria, crie `src/commands/<categoria>/mod.rs`
/// e inclua-a aqui.
pub fn all() -> Vec<poise::Command<Data, Error>> {
    let mut commands = Vec::new();

    commands.extend(info::commands());
    commands.extend(admin::commands());

    commands
}
