mod ban_member;
mod invite;
mod leave;
mod members;

use crate::client::{Data, Error};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        members::members(),
        ban_member::ban_member(),
        invite::invite(),
        leave::leave(),
    ]
}
