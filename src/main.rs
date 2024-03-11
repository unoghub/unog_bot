// TODO: change .ok_or_else to anyhow's .context

mod color;
mod interaction;
mod model;
mod sheets;

use std::{
    env, io,
    io::Write,
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use anyhow::{anyhow, Result};
use futures_util::stream::StreamExt;
use tracing::{error, info, warn};
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, util::SubscriberInitExt};
use twilight_gateway::{stream::ShardEventStream, Event, Intents, Shard};
use twilight_model::{
    http::attachment::Attachment,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, RoleMarker, WebhookMarker},
        Id,
    },
};
use twilight_util::link::webhook;

use crate::sheets::Sheets;

#[derive(Clone)]
struct WebhookWriter {
    ctx: Context,
    enabled: Arc<AtomicBool>,
    webhook_id: Id<WebhookMarker>,
    webhook_token: String,
}

impl WebhookWriter {
    fn new(ctx: Context, webhook_url: &str) -> Result<Self> {
        let (webhook_id, Some(webhook_token)) = webhook::parse(webhook_url)? else {
            return Err(anyhow!("provided webhook url doesn't contain a token"));
        };

        Ok(Self {
            ctx,
            enabled: Arc::new(AtomicBool::new(true)),
            webhook_id,
            webhook_token: webhook_token.to_owned(),
        })
    }

    async fn execute_webhook(self, bytes: Vec<u8>) -> Result<()> {
        self.ctx
            .client
            .execute_webhook(self.webhook_id, &self.webhook_token)
            .content("ÜNOG Bot tracing")?
            .attachments(&[Attachment::from_bytes(
                "unog_bot_tracing.rust".to_owned(),
                bytes,
                1,
            )])?
            .await?;

        Ok(())
    }
}

impl Write for &WebhookWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if !self.enabled.load(Ordering::Acquire) {
            return Ok(0);
        }

        let writer_clone = self.clone();
        let writer_err_clone = self.clone();
        let buf_vec = buf.to_vec();
        tokio::spawn(async move {
            if let Err(err) = writer_clone.execute_webhook(buf_vec).await {
                warn!(
                    ?err,
                    "couldn't execute tracing webhook, disabling webhook tracing"
                );
                writer_err_clone.enabled.store(false, Ordering::Release);
            }
        });

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for WebhookWriter {
    type Writer = &'a Self;

    fn make_writer(&'a self) -> Self::Writer {
        self
    }
}

struct Config {
    guild_id: Id<GuildMarker>,
    sheet_id: String,
    token: String,
    tracing_webhook_url: String,
    verification_submissions_channel_id: Id<ChannelMarker>,
    verified_role_id: Id<RoleMarker>,
}

impl Config {
    fn new() -> Result<Self> {
        dotenvy::dotenv()?;
        Ok(Self {
            guild_id: env::var("GUILD_ID")?.parse()?,
            sheet_id: env::var("SHEET_ID")?,
            token: env::var("TOKEN")?,
            tracing_webhook_url: env::var("TRACING_WEBHOOK_URL")?,
            verification_submissions_channel_id: env::var("VERIFICATION_SUBMISSIONS_CHANNEL_ID")?
                .parse()?,
            verified_role_id: env::var("VERIFIED_ROLE_ID")?.parse()?,
        })
    }
}

struct ContextInner {
    application_id: Id<ApplicationMarker>,
    client: twilight_http::Client,
    config: Config,
    sheets: Sheets,
}

#[derive(Clone)]
struct Context(Arc<ContextInner>);

impl Deref for Context {
    type Target = Arc<ContextInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Context {
    async fn new() -> Result<Self> {
        let config = Config::new()?;
        let client = twilight_http::Client::new(config.token.clone());
        let sheets = Sheets::new(config.sheet_id.clone()).await?;

        let application_id = client.current_user_application().await?.model().await?.id;

        Ok(Self(Arc::new(ContextInner {
            application_id,
            client,
            config,
            sheets,
        })))
    }

    async fn shards(self) -> Result<Vec<Shard>> {
        Ok(twilight_gateway::stream::create_recommended(
            &self.client,
            twilight_gateway::Config::new(self.config.token.clone(), Intents::empty()),
            |_, builder| builder.build(),
        )
        .await?
        .collect())
    }

    async fn handle_event(self, event: Event) {
        let event_handle_res: Result<()> = match event {
            Event::Ready(_) => {
                info!("ready set go");
                Ok(())
            }
            Event::InteractionCreate(interaction) => self.handle_interaction(interaction.0).await,
            _ => Ok(()),
        };

        if let Err(err) = event_handle_res {
            warn!(?err);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = Context::new().await?;

    let webhook_writer = WebhookWriter::new(ctx.clone(), &ctx.config.tracing_webhook_url)?;
    let fmt_tracing_layer = tracing_subscriber::fmt::layer().without_time().pretty();
    let webhook_tracing_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_ansi(false)
        .pretty()
        .with_writer(webhook_writer);
    tracing_subscriber::registry()
        .with(fmt_tracing_layer)
        .with(webhook_tracing_layer)
        .with(tracing_subscriber::EnvFilter::try_from_default_env()?)
        .try_init()?;

    ctx.set_commands().await?;

    let mut shards = ctx.clone().shards().await?;
    let mut event_stream = ShardEventStream::new(shards.iter_mut());

    while let Some((_, event_res)) = event_stream.next().await {
        match event_res {
            Ok(event) => {
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    ctx_clone.handle_event(event).await;
                })
            }
            Err(err) => {
                warn!(?err, "error receiving event");

                if err.is_fatal() {
                    error!("received fatal error, exiting");
                    break;
                }

                continue;
            }
        };
    }

    Ok(())
}
