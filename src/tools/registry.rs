use rust_mcp_sdk::schema::{CallToolError, CallToolRequestParams, ListToolsResult, Tool};
use serde_json::Map;

use super::{ToolContext, ToolError, ToolResult};

pub type ToolHandler =
    fn(Map<String, serde_json::Value>, &dyn ToolContext) -> Result<ToolResult, ToolError>;

pub struct ToolRegistry {
    tools: Vec<ToolDefinition>,
}

struct ToolDefinition {
    tool: Tool,
    handler: ToolHandler,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register(&mut self, tool: Tool, handler: ToolHandler) {
        self.tools.push(ToolDefinition { tool, handler });
    }

    pub fn list_tools(&self) -> ListToolsResult {
        ListToolsResult {
            tools: self.tools.iter().map(|t| t.tool.clone()).collect(),
            meta: None,
            next_cursor: None,
        }
    }

    pub fn dispatch(
        &self,
        params: CallToolRequestParams,
        ctx: &dyn ToolContext,
    ) -> Result<ToolResult, CallToolError> {
        let args = params.arguments.unwrap_or_default();
        let def = self
            .tools
            .iter()
            .find(|t| t.tool.name == params.name)
            .ok_or_else(|| CallToolError::unknown_tool(params.name.clone()))?;
        (def.handler)(args, ctx).map_err(|e| CallToolError::from_message(e.to_string()))
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
