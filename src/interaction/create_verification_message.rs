use anyhow::{anyhow, Result};
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::Interaction,
    },
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component, MessageFlags, ReactionType,
    },
    guild::Permissions,
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::ChannelMarker, Id},
};
use twilight_util::builder::{
    command::CommandBuilder, embed::EmbedBuilder, InteractionResponseDataBuilder,
};

use crate::{
    color::Color,
    interaction::{
        show_verification_modal::ShowVerificationModal, CreateCommand, InteractionContext,
        RunInteraction,
    },
};

pub struct CreateVerificationMessage {
    channel_id: Id<ChannelMarker>,
    ctx: InteractionContext,
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

    async fn new(interaction: Interaction, ctx: InteractionContext) -> Result<Self> {
        Ok(Self {
            channel_id: interaction
                .channel
                .as_ref()
                .ok_or_else(|| {
                    anyhow!("create_verification_message interaction doesn't have channel attached")
                })?
                .id,
            ctx,
        })
    }

    async fn run(self) -> Result<()> {
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

        self.ctx
            .core
            .client
            .create_message(self.channel_id)
            .components(&[Component::ActionRow(ActionRow {
                components: vec![Component::Button(show_verification_modal_button)],
            })])?
            .await?;

        let response_embed = EmbedBuilder::new()
            .title("📨 Doğrulanma mesajı atıldı")
            .description(
                "Kullanıcılar bu mesajdaki butonu kullanarak doğrulanma formunu açabilecek.",
            )
            .color(Color::Success.into())
            .build();

        self.ctx
            .create_response(&InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .embeds([response_embed])
                        .flags(MessageFlags::EPHEMERAL)
                        .build(),
                ),
            })
            .await?;

        Ok(())
    }
}
