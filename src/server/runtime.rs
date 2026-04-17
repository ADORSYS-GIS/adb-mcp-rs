use rust_mcp_sdk::{
    McpServer, McpServerHandler, StdioTransport, ToMcpServerHandler, TransportOptions,
    error::SdkResult,
    id_generator::UuidGenerator,
    mcp_http::McpHttpHandler,
    mcp_server::{HyperServerOptions, McpAppState, McpServerOptions, server_runtime},
    schema::InitializeResult,
    session_store::InMemorySessionStore,
};
use std::sync::Arc;
use std::time::Duration;

use super::handler::AdbHandler;
use super::http_cors::CorsServer;
use super::info::ServerInfoBuilder;
use crate::cli::{Cli, Mode};

pub struct ServerRuntime;

impl ServerRuntime {
    pub async fn start(cli: &Cli) -> SdkResult<()> {
        let server_info = ServerInfoBuilder::new().build();
        let handler = AdbHandler::new().to_mcp_server_handler();

        match cli.transport_mode {
            Mode::Http => Self::start_http(server_info, handler, cli.port).await,
            Mode::Stdio => Self::start_stdio(server_info, handler).await,
        }
    }

    async fn start_stdio(
        server_info: InitializeResult,
        handler: Arc<dyn McpServerHandler>,
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
        server_info: InitializeResult,
        handler: Arc<dyn McpServerHandler>,
        port: u16,
    ) -> SdkResult<()> {
        use rust_mcp_sdk::event_store::InMemoryEventStore;

        let state = Arc::new(McpAppState {
            session_store: Arc::new(InMemorySessionStore::new()),
            id_generator: Arc::new(UuidGenerator {}),
            stream_id_gen: Arc::new(rust_mcp_sdk::id_generator::FastIdGenerator::new(Some("s_"))),
            server_details: Arc::new(server_info),
            handler,
            ping_interval: Duration::from_secs(12),
            transport_options: Arc::new(TransportOptions::default()),
            enable_json_response: false,
            event_store: Some(Arc::new(InMemoryEventStore::default())),
            task_store: None,
            client_task_store: None,
            message_observer: None,
        });

        let http_handler = Arc::new(McpHttpHandler::new(vec![], None));

        let options = HyperServerOptions {
            host: "0.0.0.0".to_string(),
            port,
            sse_support: true,
            health_endpoint: Some("/health".to_string()),
            event_store: Some(Arc::new(InMemoryEventStore::default())),
            ..Default::default()
        };

        CorsServer::start(state, http_handler, options).await
    }
}
