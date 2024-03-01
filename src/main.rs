use std::{env, sync::Arc};

use anyhow::Result;
use futures_util::stream::StreamExt;
use tracing::{error, warn};
use twilight_gateway::{stream::ShardEventStream, Event, Intents, Shard};

struct Config {
    token: String,
}

impl Config {
    fn new() -> Result<Self> {
        dotenvy::dotenv()?;
        Ok(Self {
            token: env::var("TOKEN")?,
        })
    }
}

struct Context {
    client: twilight_http::Client,
    config: Config,
}

impl Context {
    fn new() -> Result<Self> {
        let config = Config::new()?;
        let client = twilight_http::Client::new(config.token.clone());

        Ok(Self { client, config })
    }

    async fn shards(&self) -> Result<Vec<Shard>> {
        Ok(twilight_gateway::stream::create_recommended(
            &self.client,
            twilight_gateway::Config::new(self.config.token.clone(), Intents::empty()),
            |_, builder| builder.build(),
        )
        .await?
        .collect())
    }

    async fn handle_event(&self, event: Event) {
        let event_handle_res: Result<()> = match event {
            _ => Ok(()),
        };

        if let Err(err) = event_handle_res {
            warn!(?err, "couldn't handle event")
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let ctx = Arc::new(Context::new()?);

    let mut shards = ctx.shards().await?;
    let mut event_stream = ShardEventStream::new(shards.iter_mut());

    while let Some((_, event_res)) = event_stream.next().await {
        match event_res {
            Ok(event) => {
                let ctx_ref = Arc::clone(&ctx);
                tokio::spawn(async move {
                    ctx_ref.handle_event(event).await;
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
