use rust_mcp_sdk::{
    McpServer, StdioTransport, ToMcpServerHandler, TransportOptions,
    error::SdkResult,
    mcp_server::{HyperServerOptions, McpServerOptions, hyper_server, server_runtime},
};
use std::sync::Arc;

use super::handler::AdbHandler;
use super::info::ServerInfoBuilder;
use crate::cli::Cli;

pub struct ServerRuntime;

impl ServerRuntime {
    pub async fn start(cli: &Cli) -> SdkResult<()> {
        let server_info = ServerInfoBuilder::new().build();
        let handler = AdbHandler::new().to_mcp_server_handler();

        match cli.mode.as_str() {
            "http" => Self::start_http(server_info, handler, cli.port).await,
            _ => Self::start_stdio(server_info, handler).await,
        }
    }

    async fn start_stdio(
        server_info: rust_mcp_sdk::schema::InitializeResult,
        handler: Arc<dyn rust_mcp_sdk::McpServerHandler>,
    ) -> SdkResult<()> {
        tracing::info!("Starting ADB MCP Server (stdio mode)");

        let transport = StdioTransport::new(TransportOptions::default())?;
        let server = server_runtime::create_server(McpServerOptions {
            server_details: server_info,
            transport,
            handler,
            task_store: None,
            client_task_store: None,
            message_observer: None,
        });

        server.start().await
    }

    async fn start_http(
        server_info: rust_mcp_sdk::schema::InitializeResult,
        handler: Arc<dyn rust_mcp_sdk::McpServerHandler>,
        port: u16,
    ) -> SdkResult<()> {
        tracing::info!("Starting ADB MCP Server on http://127.0.0.1:{}", port);

        let server = hyper_server::create_server(
            server_info,
            handler,
            HyperServerOptions {
                host: "127.0.0.1".to_string(),
                port,
                event_store: Some(Arc::new(
                    rust_mcp_sdk::event_store::InMemoryEventStore::default(),
                )),
                sse_support: true,
                ..Default::default()
            },
        );

        server.start().await
    }
}
