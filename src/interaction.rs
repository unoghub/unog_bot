use anyhow::{anyhow, bail, Result};
use twilight_http::client::InteractionClient;
use twilight_model::application::{
    command::Command,
    interaction::{Interaction, InteractionData},
};

use crate::{verification::create_verification_message::CreateVerificationMessage, Context};

pub trait CreateCommand {
    fn command() -> Result<Command>;
}

#[allow(clippy::module_name_repetitions)]
pub trait RunInteraction: Sized {
    const CUSTOM_ID: &'static str;

    async fn new(ctx: &Context, interaction: Interaction) -> Result<Self>;

    async fn run(self, ctx: &Context) -> Result<()>;
}

impl Context {
    pub const fn interaction_client(&self) -> InteractionClient<'_> {
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

    pub async fn handle_interaction(&self, interaction: Interaction) -> Result<()> {
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
                CreateVerificationMessage::new(self, interaction)
                    .await?
                    .run(self)
                    .await?;
            }
            _ => bail!("unknown interaction custom id: {custom_id}"),
        }

        Ok(())
    }
}
