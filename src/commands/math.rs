use serenity::{
    client::Context,
    model::interactions::application_command::ApplicationCommandInteraction,
    prelude::{SerenityError},
};

use parrot::utils::create_response;
use crate::strings::MATH_NO_VALID_EXPR;

pub async fn eval(
    ctx: &Context,
    interaction: &mut ApplicationCommandInteraction,
) -> Result<(), SerenityError> {
    let args = interaction.data.options.clone();

    let expr = match args.first() {
        Some(t) if t.value.is_some() => t.value.as_ref().unwrap().as_str().unwrap(),
        _ => return create_response(&ctx.http, interaction, MATH_NO_VALID_EXPR).await,
    };

    let mut ns = |name:&str, params:Vec<f64>| -> Option<f64> {
        match name {
            // Functions:
            "sqrt" => Some(params[0].sqrt()),

            // A wildcard to handle all undefined names:
            _ => None,
        }
    };

    let result = fasteval::ez_eval(expr, &mut ns).unwrap();

    create_response(&ctx.http, interaction, format!("{}", result).as_str()).await
}
