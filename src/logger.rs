use rmcp::{
    handler::server::tool::{Parameters, ToolRouter},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ServerHandler,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;

#[async_trait::async_trait]
pub trait Logger: Send + Sync + 'static {
    async fn log(&self, role: &str, message: &str, context: Option<&str>) -> anyhow::Result<()>;
}

pub struct ConversationLogger<L> {
    logger: L,
    tool_router: ToolRouter<Self>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct LogConversationRequest {
    #[schemars(description = "The role of the message sender: 'human', 'assistant', or 'system'")]
    role: String,
    #[schemars(description = "The message content to log")]
    message: String,
    #[schemars(description = "Optional context or metadata about the message")]
    context: Option<String>,
}

#[tool_router]
impl<L> ConversationLogger<L>
where
    L: Logger,
{
    pub fn new(logger: L) -> Self {
        Self {
            logger,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Log conversation messages to Discord for review and history tracking. Use this to record important interactions, decisions, or context that should be preserved"
    )]
    async fn log_conversation(
        &self,
        Parameters(LogConversationRequest {
            role,
            message,
            context,
        }): Parameters<LogConversationRequest>,
    ) -> Result<String, rmcp::Error> {
        self.logger
            .log(&role, &message, context.as_deref())
            .await
            .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))?;
        Ok("Message logged successfully".to_string())
    }
}

#[tool_handler]
impl<L> ServerHandler for ConversationLogger<L>
where
    L: Logger,
{
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "This is a Discord Conversation Logger MCP server that enables AI assistants to \
                 log important conversations to Discord for review and history tracking. Use the \
                 'log_conversation' tool to record:\n\
                 - User requests and requirements (role='human')\n\
                 - Your responses and implementations (role='assistant')\n\
                 - Important decisions or milestones\n\
                 - Errors or issues encountered (role='system')\n\
                 \n\
                 Logged messages will be formatted with role indicators, timestamps, and organized \
                 in a dedicated Discord thread for easy review."
                    .to_string(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
