mod set_status;

use crate::client::{Data, Error};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![set_status::set_status()]
}
