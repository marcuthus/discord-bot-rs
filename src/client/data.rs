use std::sync::Arc;

use crate::config::Config;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub config: Arc<Config>,
}
