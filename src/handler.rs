use std::sync::Arc;

use serenity::{
    async_trait,
    model::prelude::{
        command::Command,
        interaction::{
            Interaction,
        },
        EmojiId, Message, ReactionType, Ready, ResumedEvent,
    },
    prelude::{Context, EventHandler},
};
use tracing::{
    info, instrument,
    log::{debug, error},
};

use crate::{
    commands::{SlashCommand},
};

pub struct BotHandler {
    pub commands: Vec<Arc<dyn SlashCommand>>,
}

pub struct Configuration;

impl BotHandler {
    pub fn new(
        commands: &[Arc<dyn SlashCommand>],
    ) -> BotHandler {
        BotHandler {
            commands: commands.into(),
        }
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command_interaction) = interaction {
            debug!("Received command interaction: {:#?}", command_interaction);

            if command_interaction.guild_id.is_none() {
                return;
            }

            let conf = Configuration;

            for command in self
                .commands
                .iter()
                .filter(|x| x.name() == command_interaction.data.name)
            {
                if let Err(why) = command
                    .dispatch(&command_interaction, &ctx, &conf)
                    .await
                {
                    error!("Error during command interaction: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_commands =
            Command::set_global_application_commands(&ctx.http, |command_builder| {
                for command in &self.commands {
                    command.register(command_builder);
                }

                command_builder
            })
            .await;

        if let Err(why) = guild_commands {
            error!("Failed to create slash commands. {}", why);
        }
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(why) = react_to_messages(ctx, msg).await {
            error!("Error while reacting to message: {}", why);
        }
    }
}

pub async fn react_to_messages(ctx: Context, msg: Message) -> Result<(), serenity::Error> {
    if msg.content.to_lowercase().contains("stalweidism") {
        msg.react(
            &ctx,
            ReactionType::Custom {
                animated: false,
                id: EmojiId(767402279539441684),
                name: Some(String::from("doy")),
            },
        )
        .await?;

        msg.react(
            &ctx,
            ReactionType::Custom {
                animated: false,
                id: EmojiId(848665642713874472),
                name: Some(String::from("FGuOoDoY")),
            },
        )
        .await?;

        info!("Reacted to message.");
    }

    Ok(())
}
