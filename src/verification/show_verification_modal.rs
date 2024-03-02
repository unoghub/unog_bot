use anyhow::Result;
use twilight_model::application::interaction::Interaction;

use crate::interaction::{InteractionContext, RunInteraction};

pub struct ShowVerificationModal;

impl RunInteraction for ShowVerificationModal {
    const CUSTOM_ID: &'static str = "show-verification-modal";

    #[allow(let_underscore_drop, clippy::let_underscore_untyped)]
    async fn new(_: Interaction, _: InteractionContext) -> Result<Self> {
        Ok(Self)
    }

    async fn run(self) -> Result<()> {
        Ok(())
    }
}
