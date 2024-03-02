use anyhow::{anyhow, bail, Result};
use twilight_http::client::InteractionClient;
use twilight_model::{
    application::{
        command::Command,
        interaction::{Interaction, InteractionData},
    },
    http::interaction::InteractionResponse,
    id::{marker::InteractionMarker, Id},
};

use crate::{
    verification::{
        create_verification_message::CreateVerificationMessage,
        show_verification_modal::ShowVerificationModal,
        verification_modal_submit::VerificationModalSubmit,
    },
    Context,
};

pub trait CreateCommand {
    fn command() -> Result<Command>;
}

#[allow(clippy::module_name_repetitions)]
pub trait RunInteraction: Sized {
    const CUSTOM_ID: &'static str;

    async fn new(interaction: Interaction, ctx: InteractionContext) -> Result<Self>;

    async fn run(self) -> Result<()>;
}

#[derive(Clone)]
pub struct InteractionContext {
    pub core: Context,
    id: Id<InteractionMarker>,
    token: String,
}

impl InteractionContext {
    pub fn new(ctx: Context, interaction: &Interaction) -> Self {
        Self {
            core: ctx,
            id: interaction.id,
            token: interaction.token.clone(),
        }
    }

    pub async fn create_response(self, response: &InteractionResponse) -> Result<()> {
        self.core
            .interaction_client()
            .create_response(self.id, &self.token, response)
            .await?;
        Ok(())
    }
}

impl Context {
    pub async fn handle_interaction(self, interaction: Interaction) -> Result<()> {
        let ctx = InteractionContext::new(self, &interaction);

        let interaction_data = interaction.data.clone().ok_or_else(|| {
            anyhow!(
                "interaction data is not  `ApplicationCommand`, `MessageComponent`, \
                 `ApplicationCommandAutocomplete` or `ModalSubmit`"
            )
        })?;

        let custom_id = match interaction_data {
            InteractionData::ApplicationCommand(data) => data.name,
            InteractionData::MessageComponent(data) => data.custom_id,
            InteractionData::ModalSubmit(data) => data.custom_id,
            _ => bail!("unknown interaction data kind"),
        };

        match custom_id.as_str() {
            CreateVerificationMessage::CUSTOM_ID => {
                CreateVerificationMessage::new(interaction, ctx)
                    .await?
                    .run()
                    .await?;
            }
            ShowVerificationModal::CUSTOM_ID => {
                ShowVerificationModal::new(interaction, ctx)
                    .await?
                    .run()
                    .await?;
            }
            VerificationModalSubmit::CUSTOM_ID => {
                VerificationModalSubmit::new(interaction, ctx)
                    .await?
                    .run()
                    .await?;
            }
            _ => bail!("unknown interaction custom id: {custom_id}"),
        }

        Ok(())
    }

    pub fn interaction_client(&self) -> InteractionClient<'_> {
        self.client.interaction(self.application_id)
    }

    pub async fn set_commands(&self) -> Result<()> {
        self.interaction_client()
            .set_guild_commands(
                self.config.guild_id,
                &[CreateVerificationMessage::command()?],
            )
            .await?;

        Ok(())
    }
}
