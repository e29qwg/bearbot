use crate::utils::ShardManagerContainer;
use serenity::http::Http;
use songbird::SerenityInit;

use std::{
    collections::{HashSet, HashMap},
    env,
    error::Error,
};

use crate::{
    events::serenity_handler::SerenityHandler,
    settings::GuildSettingsMap,
};

pub struct Client {
    client: serenity::Client,
}

impl Client {
    pub async fn default() -> Result<Client, Box<dyn Error>> {
        let token = env::var("DISCORD_TOKEN")?;
        Client::new(token).await
    }

    pub async fn new(token: String) -> Result<Client, Box<dyn Error>> {
        let http = Http::new_with_token(&token);
        let application_id = env::var("DISCORD_APPID")?.parse()?;

        // We will fetch your bot's owners and id
        let (_owners, _bot_id) = match http.get_current_application_info().await {
            Ok(info) => {
                let mut owners = HashSet::new();
                owners.insert(info.owner.id);

                (owners, info.id)
            },
            Err(why) => panic!("Could not access application info: {:?}", why),
        };

        let client = serenity::Client::builder(token)
            .event_handler(SerenityHandler)
            .application_id(application_id)
            .register_songbird()
            .await?;
        
        {
            let mut data = client.data.write().await;
            data.insert::<ShardManagerContainer>(client.shard_manager.clone());
            data.insert::<GuildSettingsMap>(HashMap::default());
            drop(data);
        }
        
        let shard_manager = client.shard_manager.clone();
        
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.expect("Could not register Ctrl+C handler");
            shard_manager.lock().await.shutdown_all().await;
        });

        Ok(Client { client })
    }

    pub async fn start(&mut self) -> Result<(), serenity::Error> {
        self.client.start_autosharded().await
    }
}