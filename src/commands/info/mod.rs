mod menu;
mod ping;
mod server_id;
mod user_id;

use crate::client::{Data, Error};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        ping::ping(),
        menu::menu(),
        user_id::user_id(),
        server_id::server_id(),
    ]
}
