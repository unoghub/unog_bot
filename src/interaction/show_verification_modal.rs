use anyhow::Result;
use twilight_model::{
    application::interaction::Interaction,
    channel::message::{
        component::{ActionRow, TextInput, TextInputStyle},
        Component,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::interaction::{
    verification_modal_submit::VerificationModalSubmit, InteractionContext, RunInteraction,
};

pub struct ShowVerificationModal {
    ctx: InteractionContext,
}

impl RunInteraction for ShowVerificationModal {
    const CUSTOM_ID: &'static str = "show-verification-modal";

    #[allow(let_underscore_drop, clippy::let_underscore_untyped)]
    async fn new(_: Interaction, ctx: InteractionContext) -> Result<Self> {
        Ok(Self { ctx })
    }

    async fn run(self) -> Result<()> {
        let name_surname_input = Component::TextInput(TextInput {
            custom_id: "name-surname".to_owned(),
            label: "İSİM SOYİSİM".to_owned(),
            max_length: Some(32),
            min_length: None,
            placeholder: None,
            required: None,
            style: TextInputStyle::Short,
            value: None,
        });
        let email_input = Component::TextInput(TextInput {
            custom_id: "email".to_owned(),
            label: "E-POSTA ADRESİ".to_owned(),
            max_length: Some(254),
            min_length: None,
            placeholder: None,
            required: None,
            style: TextInputStyle::Short,
            value: None,
        });
        let birthday_input = Component::TextInput(TextInput {
            custom_id: "birthday".to_owned(),
            label: "DOĞUM TARİHİ".to_owned(),
            max_length: Some(10),
            min_length: None,
            placeholder: Some("GG.AA.YYYY".to_owned()),
            required: None,
            style: TextInputStyle::Short,
            value: None,
        });
        let experience_input = Component::TextInput(TextInput {
            custom_id: "experience".to_owned(),
            label: "KAÇ YILDIR OYUN SEKTÖRÜNDESİNİZ?".to_owned(),
            max_length: Some(2),
            min_length: None,
            placeholder: None,
            required: None,
            style: TextInputStyle::Short,
            value: None,
        });
        let organization_input = Component::TextInput(TextInput {
            custom_id: "organization".to_owned(),
            label: "BULUNDUĞUNUZ KURUM VEYA EKİP".to_owned(),
            max_length: Some(100),
            min_length: None,
            placeholder: None,
            required: Some(false),
            style: TextInputStyle::Short,
            value: None,
        });

        let rows = [
            Component::ActionRow(ActionRow {
                components: vec![name_surname_input],
            }),
            Component::ActionRow(ActionRow {
                components: vec![email_input],
            }),
            Component::ActionRow(ActionRow {
                components: vec![birthday_input],
            }),
            Component::ActionRow(ActionRow {
                components: vec![experience_input],
            }),
            Component::ActionRow(ActionRow {
                components: vec![organization_input],
            }),
        ];

        let response = InteractionResponseDataBuilder::new()
            .custom_id(VerificationModalSubmit::CUSTOM_ID)
            .title("📝 Doğrulanma Formu")
            .components(rows)
            .build();

        self.ctx
            .create_response(&InteractionResponse {
                kind: InteractionResponseType::Modal,
                data: Some(response),
            })
            .await?;

        Ok(())
    }
}
