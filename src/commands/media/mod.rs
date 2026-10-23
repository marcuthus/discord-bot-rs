mod create_sticker;

use crate::client::{Data, Error};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![create_sticker::create_sticker()]
}
