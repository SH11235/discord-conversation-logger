use std::sync::{Arc, OnceLock};

use serenity::{
    all::{
        AutoArchiveDuration, ChannelId, ChannelType, Context, CreateEmbed, CreateMessage,
        CreateThread, EventHandler, GatewayIntents, Ready,
    },
    Client,
};
use tokio::sync::OnceCell;

use crate::logger::Logger;

pub async fn start(discord_token: &str, handler: Handler) -> anyhow::Result<()> {
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(discord_token, intents)
        .event_handler(handler)
        .await?;
    Ok(client.start().await?)
}

#[derive(Clone)]
pub struct Handler {
    ctx: Arc<OnceLock<Context>>,
}

impl Default for Handler {
    fn default() -> Self {
        Self {
            ctx: Arc::new(OnceLock::new()),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        self.ctx.set(ctx).ok();
    }
}

pub struct DiscordLogger {
    log_channel_id: ChannelId,
    log_thread_name: String,
    handler: Handler,
    log_thread: OnceCell<ChannelId>,
}

impl DiscordLogger {
    pub fn new(log_channel_id: ChannelId, log_thread_name: String) -> Self {
        Self {
            log_channel_id,
            log_thread_name,
            handler: Handler::default(),
            log_thread: OnceCell::new(),
        }
    }

    pub fn handler(&self) -> &Handler {
        &self.handler
    }
}

#[async_trait::async_trait]
impl Logger for DiscordLogger {
    async fn log(&self, role: &str, message: &str, context: Option<&str>) -> anyhow::Result<()> {
        let ctx = self
            .handler
            .ctx
            .get()
            .ok_or_else(|| anyhow::anyhow!("The connection with Discord is not ready"))?;

        let log_thread = self
            .log_thread
            .get_or_try_init(|| async {
                let thread = self
                    .log_channel_id
                    .create_thread(
                        &ctx.http,
                        CreateThread::new(&self.log_thread_name)
                            .auto_archive_duration(AutoArchiveDuration::OneWeek)
                            .kind(ChannelType::PublicThread),
                    )
                    .await?;
                anyhow::Ok(thread.id)
            })
            .await?;

        let color = match role {
            "human" => 0x3498db,     // Blue
            "assistant" => 0x2ecc71, // Green
            "system" => 0x95a5a6,    // Gray
            _ => 0x7f8c8d,           // Default gray
        };

        let mut embed = CreateEmbed::new()
            .title(format!("💬 {}", role.to_uppercase()))
            .description(message)
            .color(color)
            .timestamp(serenity::all::Timestamp::now());

        if let Some(ctx_info) = context {
            embed = embed.footer(serenity::all::CreateEmbedFooter::new(ctx_info));
        }

        log_thread
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await?;

        Ok(())
    }
}
