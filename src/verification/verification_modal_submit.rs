use anyhow::{anyhow, bail, Result};
use twilight_model::{
    application::interaction::{
        modal::ModalInteractionDataActionRow, Interaction, InteractionData,
    },
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component, ReactionType,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::UserMarker, Id},
};
use twilight_util::builder::{
    embed::{EmbedBuilder, EmbedFieldBuilder},
    InteractionResponseDataBuilder,
};

use crate::{
    color::Color,
    interaction::{InteractionContext, RunInteraction},
};

#[derive(Clone)]
pub struct VerificationModalSubmit {
    user_id: Id<UserMarker>,
    birthday: String,
    ctx: InteractionContext,
    email: String,
    experience: String,
    name_surname: String,
    organisation: String,
}

impl VerificationModalSubmit {
    fn find_component_value(
        components: &[ModalInteractionDataActionRow],
        custom_id: &str,
    ) -> Result<String> {
        components
            .iter()
            .find_map(|row| {
                row.components
                    .first()
                    .and_then(|component| component.value.clone())
            })
            .ok_or_else(|| anyhow!("couldn't find {custom_id} in verification modal"))
    }

    async fn create_verification_submission_message(self) -> Result<()> {
        let embed = EmbedBuilder::new()
            .title("❔ Doğrulanma formu dolduruldu")
            .field(EmbedFieldBuilder::new(
                "Kullanıcı",
                format!("<@{}>", self.user_id),
            ))
            .field(EmbedFieldBuilder::new("İsim Soyisim", self.name_surname))
            .field(EmbedFieldBuilder::new("E-Posta Adresi", self.email))
            .field(EmbedFieldBuilder::new("Doğum Tarihi", self.birthday))
            .field(EmbedFieldBuilder::new("Deneyim Yılı", self.experience))
            .field(EmbedFieldBuilder::new("Kurum veya Ekip", self.organisation))
            .color(Color::Pending.into())
            .build();

        let approve_button = Component::Button(Button {
            custom_id: Some("approve-verification".to_owned()),
            disabled: false,
            emoji: Some(ReactionType::Unicode {
                name: "✅".to_owned(),
            }),
            label: Some("Doğrula".to_owned()),
            style: ButtonStyle::Success,
            url: None,
        });

        self.ctx
            .core
            .client
            .create_message(self.ctx.core.config.verification_submissions_channel_id)
            .embeds(&[embed])?
            .components(&[Component::ActionRow(ActionRow {
                components: vec![approve_button],
            })])?
            .await?;

        Ok(())
    }

    async fn respond(self) -> Result<()> {
        let response_embed = EmbedBuilder::new()
            .title("📨 Doğrulanma formunuz iletildi")
            .description(
                "Formunuzda bir sorun yoksa yakında doğrulanacaksınız. Şimdiden hoş geldiniz!",
            )
            .color(Color::Success.into())
            .build();

        self.ctx
            .create_response(&InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .embeds([response_embed])
                        .build(),
                ),
            })
            .await?;

        Ok(())
    }
}

impl RunInteraction for VerificationModalSubmit {
    const CUSTOM_ID: &'static str = "verification-modal-submit";

    async fn new(interaction: Interaction, ctx: InteractionContext) -> Result<Self> {
        let user_id = interaction
            .author_id()
            .ok_or_else(|| anyhow!("verification modal interaction has no user"))?;

        let InteractionData::ModalSubmit(modal) = interaction
            .data
            .ok_or_else(|| anyhow!("verification modal has no interaction data"))?
        else {
            bail!("verification modal data is not of kind modal submit")
        };
        let components = modal.components;

        Ok(Self {
            user_id,
            birthday: Self::find_component_value(&components, "birthday")?,
            ctx,
            email: Self::find_component_value(&components, "email")?,
            experience: Self::find_component_value(&components, "experience")?,
            name_surname: Self::find_component_value(&components, "name-surname")?,
            organisation: Self::find_component_value(&components, "organisation")?,
        })
    }

    async fn run(self) -> Result<()> {
        self.clone()
            .create_verification_submission_message()
            .await?;
        self.respond().await?;

        Ok(())
    }
}
