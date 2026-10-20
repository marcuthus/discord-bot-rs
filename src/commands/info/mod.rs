mod ping;

use crate::client::{Data, Error};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![ping::ping()]
}
