use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{ApplicationCommand, ApplicationCommandOptionType},
            Interaction,
        },
        prelude::{Activity, VoiceState},
    },
};

use parrot::commands::{
    autopause::*, clear::*, leave::*, now_playing::*, pause::*, play::*, playtop::*, queue::*,
    remove::*, repeat::*, resume::*, seek::*, shuffle::*, skip::*, stop::*,
};

use crate::commands::{
    math::*, meta::*,
};

use tracing::{info};

pub struct SerenityHandler;

#[async_trait]
impl EventHandler for SerenityHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("🐻 {} is connected!", ready.user.name);

        let activity = Activity::listening("/play");
        ctx.set_activity(activity).await;

        ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            // Parrot
            commands
                .create_application_command(|command| {
                    command
                        .name("autopause")
                        .description("Toggles whether to pause after a song ends")
                })
                .create_application_command(|command| {
                    command.name("clear").description("Clears the queue")
                })
                .create_application_command(|command| {
                    command
                        .name("leave")
                        .description("Leave the voice channel the bot is connected to")
                })
                .create_application_command(|command| {
                    command
                        .name("np")
                        .description("Displays information about the current track")
                })
                .create_application_command(|command| {
                    command
                        .name("pause")
                        .description("Pauses the current track")
                })
                .create_application_command(|command| {
                    command
                        .name("play")
                        .description("Adds a track to the queue")
                        .create_option(|option| {
                            option
                                .name("query")
                                .description("The media to play")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("playtop")
                        .description("Places a track on the top of the queue")
                        .create_option(|option| {
                            option
                                .name("query")
                                .description("The media to play")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command.name("queue").description("Shows the queue")
                })
                .create_application_command(|command| {
                    command
                        .name("remove")
                        .description("Removes a track from the queue")
                        .create_option(|option| {
                            option
                                .name("index")
                                .description("Position of the track in the queue (1 is the next track to be played)")
                                .kind(ApplicationCommandOptionType::Integer)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("repeat")
                        .description("Toggles looping for the current track")
                })
                .create_application_command(|command| {
                    command
                        .name("resume")
                        .description("Resumes the current track")
                })
                .create_application_command(|command| {
                    command
                        .name("seek")
                        .description("Seeks current track to the given position")
                        .create_option(|option| {
                            option
                                .name("timestamp")
                                .description("Timestamp in the format HH:MM:SS")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command.name("shuffle").description("Shuffles the queue")
                })
                .create_application_command(|command| {
                    command.name("skip").description("Skips the current track")
                })
                .create_application_command(|command| {
                    command
                        .name("stop")
                        .description("Stops the bot and clears the queue")
                })
                .create_application_command(|command| {
                    command
                        .name("join")
                        .description("Summons the bot in your voice channel")
                })

                // BearBot
                .create_application_command(|command| {
                    command
                        .name("eval")
                        .description("Evaluates a mathematic expression")
                        .create_option(|option| {
                            option
                                .name("expr")
                                .description("Mathematic Expression")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("version")
                        .description("Displays the current version")
                })
        })
        .await
        .unwrap();

        info!("Global Application Commands are set.");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        info!("Interaction created.");

        if let Interaction::ApplicationCommand(mut command) = interaction {
            info!("Calling {}...", command.data.name);

            match command.data.name.as_str() {
                // Parrot
                "autopause" => autopause(&ctx, &mut command).await,
                "clear" => clear(&ctx, &mut command).await,
                "leave" => leave(&ctx, &mut command).await,
                "np" => now_playing(&ctx, &mut command).await,
                "pause" => pause(&ctx, &mut command).await,
                "play" => play(&ctx, &mut command).await,
                "playtop" => playtop(&ctx, &mut command).await,
                "queue" => queue(&ctx, &mut command).await,
                "remove" => remove(&ctx, &mut command).await,
                "repeat" => repeat(&ctx, &mut command).await,
                "resume" => resume(&ctx, &mut command).await,
                "seek" => seek(&ctx, &mut command).await,
                "shuffle" => shuffle(&ctx, &mut command).await,
                "skip" => skip(&ctx, &mut command).await,
                "stop" => stop(&ctx, &mut command).await,
                "join" => parrot::commands::summon::summon(&ctx, &mut command, false).await,

                // BearBot
                "eval" => eval(&ctx, &mut command).await,
                "version" => version(&ctx, &mut command).await,
                _ => unimplemented!(),
            }
            .unwrap();
        }
    }

    async fn voice_state_update(
        &self,
        ctx: Context,
        guild: Option<GuildId>,
        _old: Option<VoiceState>,
        new: VoiceState,
    ) {
        if new.user_id == ctx.http.get_current_user().await.unwrap().id && !new.deaf {
            guild
                .unwrap()
                .edit_member(&ctx.http, new.user_id, |n| n.deafen(true))
                .await
                .unwrap();
        }
    }
}
