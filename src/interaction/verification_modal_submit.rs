use std::vec::IntoIter;

use anyhow::{anyhow, bail, Result};
use tracing::warn;
use twilight_model::{
    application::interaction::{
        modal::ModalInteractionDataActionRow, Interaction, InteractionData,
    },
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component, MessageFlags, ReactionType,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{
    embed::{EmbedBuilder, EmbedFieldBuilder},
    InteractionResponseDataBuilder,
};

use crate::{
    color::Color,
    interaction::{approve_verification::ApproveVerification, InteractionContext, RunInteraction},
    model::verification::VerificationSubmission,
};

#[derive(Clone)]
pub struct VerificationModalSubmit {
    ctx: InteractionContext,
    submission: VerificationSubmission,
}

impl VerificationModalSubmit {
    fn next_component_value(
        components: &mut IntoIter<ModalInteractionDataActionRow>,
    ) -> Result<String> {
        components
            .next()
            .and_then(|row| row.components.into_iter().next())
            .and_then(|component| component.value)
            .ok_or_else(|| anyhow!("modal components iterator is drained"))
    }

    async fn create_verification_submission_message(self) -> Result<()> {
        let embed = EmbedBuilder::new()
            .title("❔ Doğrulanma formu dolduruldu")
            .field(EmbedFieldBuilder::new(
                "Kullanıcı",
                format!("<@{}>", self.submission.user_id),
            ))
            .field(EmbedFieldBuilder::new(
                "İsim Soyisim",
                self.submission.name_surname,
            ))
            .field(EmbedFieldBuilder::new(
                "E-Posta Adresi",
                self.submission.email,
            ))
            .field(EmbedFieldBuilder::new(
                "Doğum Tarihi",
                self.submission.birthday,
            ))
            .field(EmbedFieldBuilder::new(
                "Yıllık Oyun Sektörü Tecrübesi",
                self.submission.experience,
            ))
            .field(EmbedFieldBuilder::new(
                "Kurum veya Ekip",
                self.submission.organization,
            ))
            .color(Color::Pending.into())
            .build();

        let approve_button = Component::Button(Button {
            custom_id: Some(ApproveVerification::CUSTOM_ID.to_owned()),
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

    async fn append_to_sheet(self) -> Result<()> {
        self.ctx
            .core
            .sheets
            .append_verification_submission(self.submission)
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
                        .flags(MessageFlags::EPHEMERAL)
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

        let mut components = modal.components.into_iter();
        let name_surname = to_title_case(&Self::next_component_value(&mut components)?);
        let email = Self::next_component_value(&mut components)?;
        let birthday = Self::next_component_value(&mut components)?;
        let experience = Self::next_component_value(&mut components)?;
        let organization = Self::next_component_value(&mut components)?;

        Ok(Self {
            submission: VerificationSubmission {
                birthday,
                email,
                experience,
                name_surname,
                organization: if organization.is_empty() {
                    "Yok".to_owned()
                } else {
                    organization
                },
                user_id,
            },
            ctx,
        })
    }

    async fn run(self) -> Result<()> {
        if let Err(err) = self.clone().append_to_sheet().await {
            warn!(
                ?err,
                "couldn't append verification submission to sheet: {:#?}", self.submission
            );
        }

        self.clone()
            .create_verification_submission_message()
            .await?;

        self.respond().await?;

        Ok(())
    }
}

fn to_title_case(string: &str) -> String {
    let mut title = String::new();

    for word in string.split_whitespace() {
        let mut chars = word.chars();
        let Some(first_char) = chars.next() else {
            continue;
        };

        let first_char_uppercase = if first_char == 'i' {
            'İ'.to_string()
        } else {
            first_char.to_uppercase().to_string()
        };
        title.push_str(&first_char_uppercase);

        for char in chars {
            title.push_str(&char.to_lowercase().to_string());
        }

        title.push(' ');
    }

    title
}
