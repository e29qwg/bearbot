use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
};

use parrot::utils::create_response;
use serenity::prelude::SerenityError;

pub async fn version(
    ctx: &Context,
    interaction: &mut ApplicationCommandInteraction,
) -> Result<(), SerenityError> {
    let pkg_name = option_env!("CARGO_PKG_NAME").unwrap_or_else(|| "BearBot");
    let pkg_version = option_env!("CARGO_PKG_VERSION").unwrap_or_else(|| "?");
    let content = format!("{} v{}", pkg_name, pkg_version);
    
    create_response(&ctx.http, interaction, &content).await
}
