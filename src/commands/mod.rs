mod admin;
mod group;
mod info;
mod media;

use crate::client::{Data, Error};

pub fn all() -> Vec<poise::Command<Data, Error>> {
    let mut commands = Vec::new();

    commands.extend(info::commands());
    commands.extend(group::commands());
    commands.extend(media::commands());
    commands.extend(admin::commands());

    commands
}
