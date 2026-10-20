mod admin;
mod info;

use crate::client::{Data, Error};

pub fn all() -> Vec<poise::Command<Data, Error>> {
    let mut commands = Vec::new();

    commands.extend(info::commands());
    commands.extend(admin::commands());

    commands
}
