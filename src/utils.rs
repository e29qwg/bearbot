use serenity::{
    client::bridge::gateway::ShardManager,
    model::prelude::User,
    prelude::{TypeMapKey, Mutex},
};
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub fn get_full_username(user: &User) -> String {
    format!("{}#{:04}", user.name, user.discriminator)
}
