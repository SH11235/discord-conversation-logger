# Discord Conversation Logger MCP Server

An MCP (Model Context Protocol) server that allows AI assistants to log conversations to Discord for review and history tracking.

## Overview

This MCP server provides a simple way for AI assistants to maintain conversation logs in Discord. It's useful for:

- **Audit Trail**: Keep a record of important interactions and decisions
- **Review**: Easily review past conversations and context
- **Collaboration**: Share conversation history with team members
- **Debugging**: Track issues and their resolutions

## Features

- Log messages with different roles (human, assistant, system)
- Color-coded Discord embeds for easy identification
- Automatic thread creation for organized logs
- Timestamps on all messages
- Optional context metadata

## Requirements

- Rust (1.70 or higher)
- Discord account and bot
- MCP-compatible AI client (Claude Desktop, Claude Code, etc.)

## Setup

### 1. Create Discord Bot

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a new application
3. Create a bot in the Bot section and obtain the token
4. Set required permissions:
   - Send Messages
   - Create Public Threads
   - Read Message History
   - Embed Links
5. Enable privileged gateway intents in Bot section:
   - Message Content Intent

### 2. Install

```bash
cargo install --git https://github.com/SH11235/discord-conversation-logger.git
```

Or clone and build locally:

```bash
git clone https://github.com/SH11235/discord-conversation-logger.git
cd discord-conversation-logger
cargo install --path .
```

## Configuration

### Quick Setup for Claude Code

1. Run the setup script from the repository directory:
   ```bash
   ./setup-discord-env.sh
   ```

2. Edit `~/.claude/discord-config.json` with your Discord credentials:
   ```json
   {
     "bot_token": "YOUR_DISCORD_BOT_TOKEN",
     "channel_id": "YOUR_CHANNEL_ID",
     "thread_name": "Conversation Log"  // Optional, defaults to "Conversation Log"
   }
   ```

3. Load environment variables (optional for testing):
   ```bash
   source ./load-discord-env.sh
   ```

4. Claude Code will automatically detect and use this configuration when the MCP server is properly configured.

### Manual Configuration

Add the following to your MCP configuration:

```json
{
  "mcpServers": {
    "discord-conversation-logger": {
      "command": "discord-conversation-logger",
      "args": [
        "--log-channel-id", "your-channel-id",
        "--log-thread-name", "AI Conversation Log"
      ],
      "env": {
        "DISCORD_TOKEN": "your-discord-bot-token"
      }
    }
  }
}
```

### Environment Variables

- `DISCORD_TOKEN`: Your Discord bot token
- `LOG_CHANNEL_ID`: Discord channel ID for logs
- `LOG_THREAD_NAME`: Thread name (default: "Conversation Log")

## Usage

Once configured, AI assistants can use the `log_conversation` tool:

```
Assistant: I'll log this important decision.
[Uses log_conversation tool with role="assistant", message="Implemented feature X with approach Y"]
```

### Message Roles

- **human**: User messages and requests (blue)
- **assistant**: AI responses and actions (green)
- **system**: System messages, errors, or metadata (gray)

### Example Usage

```json
{
  "role": "human",
  "message": "Please implement a new authentication system",
  "context": "feature-request"
}
```

## Message Format

Logged messages appear in Discord as embedded messages with:

- **Title**: Role indicator with emoji (💬 HUMAN, 💬 ASSISTANT, 💬 SYSTEM)
- **Color**: Role-specific color coding
- **Content**: The message text
- **Timestamp**: Automatic timestamp
- **Footer**: Optional context information

## Finding Discord IDs

### Getting Channel ID
1. Enable Developer Mode in Discord (Settings → Advanced → Developer Mode)
2. Right-click on channel → "Copy ID"

## Development

To contribute or modify:

```bash
git clone https://github.com/SH11235/discord-conversation-logger.git
cd discord-conversation-logger
cargo build
cargo run -- --log-channel-id YOUR_CHANNEL_ID
```

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

Built using:
- [rmcp](https://github.com/modelcontextprotocol/rust-sdk) - Rust MCP SDK
- [serenity](https://github.com/serenity-rs/serenity) - Discord API library for Rust