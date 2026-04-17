use async_trait::async_trait;
use rust_mcp_sdk::{
    McpServer,
    mcp_server::ServerHandler,
    schema::{
        CallToolError, CallToolRequestParams, CallToolResult, ListToolsResult,
        PaginatedRequestParams, RpcError,
    },
};
use std::sync::Arc;

use crate::tools::{AdbContext, ToolRegistry, build_registry};

pub struct AdbHandler {
    registry: ToolRegistry,
}

impl AdbHandler {
    pub fn new() -> Self {
        Self {
            registry: build_registry(),
        }
    }
}

impl Default for AdbHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ServerHandler for AdbHandler {
    async fn handle_list_tools_request(
        &self,
        _request: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> Result<ListToolsResult, RpcError> {
        Ok(self.registry.list_tools())
    }

    async fn handle_call_tool_request(
        &self,
        params: CallToolRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> Result<CallToolResult, CallToolError> {
        let ctx = AdbContext::new(None);
        let result = self.registry.dispatch(params, &ctx)?;
        Ok(CallToolResult::text_content(vec![result.into()]))
    }
}
