use anyhow::{anyhow, Result};
use twilight_model::{
    application::interaction::Interaction,
    channel::message::Embed,
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{
    color::Color,
    interaction::{InteractionContext, RunInteraction},
};

pub struct ApproveVerification {
    ctx: InteractionContext,
    submission_embed: Embed,
    guild_id: Id<GuildMarker>,
    name_surname: String,
    user_id: Id<UserMarker>,
}

impl RunInteraction for ApproveVerification {
    const CUSTOM_ID: &'static str = "approve-verification";

    async fn new(interaction: Interaction, ctx: InteractionContext) -> Result<Self> {
        let guild_id = interaction
            .guild_id
            .ok_or_else(|| anyhow!("approve verification interaction doesnt have a guild id"))?;

        let message = interaction
            .message
            .ok_or_else(|| anyhow!("approve verification interaction has no message"))?;
        let submission_embed = message
            .embeds
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("approve verification interaction has no embed"))?;
        let mut embed_fields = submission_embed.fields.iter();

        let user_id = embed_fields
            .next()
            .ok_or_else(|| anyhow!("submissiom embed doesnt have a field"))?
            .value
            .as_str()
            .trim_start_matches("<@")
            .trim_end_matches('>')
            .parse()?;

        let name_surname = embed_fields
            .next()
            .ok_or_else(|| anyhow!("submission embed has only one field"))?
            .value
            .clone();

        Ok(Self {
            ctx,
            submission_embed,
            guild_id,
            name_surname,
            user_id,
        })
    }

    async fn run(self) -> Result<()> {
        self.ctx
            .core
            .client
            .update_guild_member(self.guild_id, self.user_id)
            .nick(Some(&self.name_surname))?
            .await?;

        self.ctx
            .core
            .client
            .add_guild_member_role(
                self.guild_id,
                self.user_id,
                self.ctx.core.config.verified_role_id,
            )
            .await?;

        self.ctx
            .core
            .sheets
            .set_verification_submission_approved(self.user_id)
            .await?;

        let mut embed = self.submission_embed.clone();
        embed.title = Some("✅ Kullanıcı doğrulandı".to_owned());
        embed.color = Some(Color::Success.into());
        let response = InteractionResponseDataBuilder::new()
            .embeds([embed])
            .components([]);

        self.ctx
            .create_response(&InteractionResponse {
                kind: InteractionResponseType::UpdateMessage,
                data: Some(response.build()),
            })
            .await?;

        Ok(())
    }
}
