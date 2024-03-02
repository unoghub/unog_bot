use anyhow::{anyhow, Result};
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::Interaction,
    },
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component, ReactionType,
    },
    guild::Permissions,
    id::{marker::ChannelMarker, Id},
};
use twilight_util::builder::command::CommandBuilder;

use crate::{
    interaction::{CreateCommand, RunInteraction},
    verification::show_verification_modal::ShowVerificationModal,
    Context,
};

pub struct CreateVerificationMessage {
    channel_id: Id<ChannelMarker>,
}

impl CreateCommand for CreateVerificationMessage {
    fn command() -> Result<Command> {
        Ok(CommandBuilder::new(
            Self::CUSTOM_ID,
            "Bu kanala doğrulanma mesajını at",
            CommandType::ChatInput,
        )
        .default_member_permissions(Permissions::MANAGE_GUILD)
        .validate()?
        .build())
    }
}

impl RunInteraction for CreateVerificationMessage {
    const CUSTOM_ID: &'static str = "doğrulanma_mesajını_at";

    #[allow(clippy::let_underscore_untyped)]
    async fn new(_: &Context, interaction: Interaction) -> Result<Self> {
        Ok(Self {
            channel_id: interaction
                .channel
                .ok_or_else(|| {
                    anyhow!("create_verification_message interaction doesn't have channel attached")
                })?
                .id,
        })
    }

    async fn run(self, ctx: &Context) -> Result<()> {
        let show_verification_modal_button = Button {
            custom_id: Some(ShowVerificationModal::CUSTOM_ID.to_owned()),
            disabled: false,
            emoji: Some(ReactionType::Unicode {
                name: "📝".to_owned(),
            }),
            label: Some("Doğrulanma Formunu Aç".to_owned()),
            style: ButtonStyle::Primary,
            url: None,
        };

        ctx.client
            .create_message(self.channel_id)
            .components(&[Component::ActionRow(ActionRow {
                components: vec![Component::Button(show_verification_modal_button)],
            })])?
            .await?;

        Ok(())
    }
}
