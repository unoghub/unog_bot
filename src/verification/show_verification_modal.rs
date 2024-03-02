use anyhow::Result;
use twilight_model::application::interaction::Interaction;

use crate::{interaction::RunInteraction, Context};

pub struct ShowVerificationModal;

impl RunInteraction for ShowVerificationModal {
    const CUSTOM_ID: &'static str = "show-verification-modal";

    #[allow(let_underscore_drop, clippy::let_underscore_untyped)]
    async fn new(_: &Context, _: Interaction) -> Result<Self> {
        Ok(Self)
    }

    #[allow(clippy::let_underscore_untyped)]
    async fn run(self, _: &Context) -> Result<()> {
        Ok(())
    }
}
