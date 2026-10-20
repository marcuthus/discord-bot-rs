mod client;
mod commands;
mod config;
mod events;
mod logger;
mod utils;

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let _log_guards = logger::init();
    let config = Config::from_env()?;

    client::start(config).await
}
