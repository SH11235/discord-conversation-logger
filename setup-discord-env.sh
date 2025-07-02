#!/bin/bash

# Discord Conversation Logger Setup Script
# This script sets up the Discord bot configuration for Claude Code MCP

CONFIG_DIR="$HOME/.claude"
CONFIG_FILE="$CONFIG_DIR/discord-config.json"
EXAMPLE_FILE="discord-config.example.json"

# Create .claude directory if it doesn't exist
if [ ! -d "$CONFIG_DIR" ]; then
    echo "Creating $CONFIG_DIR directory..."
    mkdir -p "$CONFIG_DIR"
fi

# Check if example file exists
if [ ! -f "$EXAMPLE_FILE" ]; then
    echo "Error: $EXAMPLE_FILE not found in current directory"
    exit 1
fi

# Check if config already exists
if [ -f "$CONFIG_FILE" ]; then
    echo "Warning: $CONFIG_FILE already exists."
    read -p "Do you want to overwrite it? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Setup cancelled."
        exit 0
    fi
fi

# Copy the example config
cp "$EXAMPLE_FILE" "$CONFIG_FILE"
echo "Created $CONFIG_FILE"

# Instructions for the user
echo
echo "Setup complete! Next steps:"
echo "1. Edit $CONFIG_FILE with your Discord bot credentials:"
echo "   - bot_token: Your Discord bot token"
echo "   - channel_id: The channel ID to log conversations"
echo "   - thread_name: Thread name for conversation logs (optional)"
echo
echo "2. Run Claude Code and the MCP will automatically use these settings"
echo
echo "For more information on getting Discord bot credentials, see:"
echo "https://discord.com/developers/applications"
