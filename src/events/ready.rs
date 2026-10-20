use poise::serenity_prelude as serenity;
use tracing::info;

use crate::client::Error;

pub async fn run(ctx: &serenity::Context, ready: &serenity::Ready) -> Result<(), Error> {
    info!(target: "client", "Successfully started as '{}'", ready.user.tag());

    let guilds = ctx.cache.guilds();
    let total_of_members: u64 = guilds
        .iter()
        .filter_map(|id| ctx.cache.guild(*id).map(|guild| guild.member_count))
        .sum();

    info!(
        target: "client",
        "I'm on '{}' servers with '{}' total members!",
        guilds.len(),
        total_of_members
    );

    Ok(())
}
