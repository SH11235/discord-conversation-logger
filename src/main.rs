mod discord;
mod logger;

use clap::Parser;
use discord::DiscordLogger;
use rmcp::serve_server;
use serenity::all::ChannelId;
use std::env;
use tokio::io::{stdin, stdout};

#[derive(Debug, Parser)]
struct Args {
    #[clap(long, env = "DISCORD_TOKEN")]
    discord_token: String,
    #[clap(long, env = "LOG_CHANNEL_ID")]
    log_channel_id: ChannelId,
    #[clap(long, env = "LOG_THREAD_NAME", default_value = "Conversation Log")]
    log_thread_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args {
        discord_token,
        log_channel_id,
        log_thread_name,
    } = Args::parse();

    // Get current working directory
    let cwd = env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // Include working directory in thread name
    let thread_name_with_cwd = format!("{} [{}]", log_thread_name, cwd);

    let discord_logger = DiscordLogger::new(log_channel_id, thread_name_with_cwd);
    let discord = discord::start(&discord_token, discord_logger.handler().clone());

    let handler = logger::ConversationLogger::new(discord_logger);
    let transport = (stdin(), stdout());
    let mcp = serve_server(handler, transport).await?;

    tokio::select! {
        res = mcp.waiting() => {
            res?;
        },
        res = discord => {
            res?;
        },
    }
    Ok(())
}
