use bearbot::client::Client;
use std::error::Error;

use tracing::{error, info};
use tracing_subscriber::{
    FmtSubscriber,
    EnvFilter,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `LOG_LEVEL` to debug`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("LOG_LEVEL"))
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to start the logger");
    
    info!("BearBot is running.");

    let mut bearbot = Client::default().await?;
    if let Err(why) = bearbot.start().await {
        error!("Crashed because: {:?}", why);
    };

    Ok(())
}
