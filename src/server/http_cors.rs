use axum::{
    Extension, Router,
    extract::{Query, State},
    response::IntoResponse,
    routing::{delete, get, post},
};
use http::{HeaderMap, Method, Uri};
use rust_mcp_sdk::{
    mcp_http::McpHttpHandler,
    mcp_server::{HyperServerOptions, McpAppState},
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tower_http::cors::{Any, CorsLayer};

pub struct CorsServer;

impl CorsServer {
    pub fn build_router(
        state: Arc<McpAppState>,
        http_handler: Arc<McpHttpHandler>,
        options: &HyperServerOptions,
    ) -> Router {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let mcp_endpoint = options.streamable_http_endpoint();
        let sse_endpoint = options.sse_endpoint();
        let messages_endpoint = options.sse_messages_endpoint();
        let health_endpoint = options.health_endpoint.as_deref();

        let mut router = Router::new()
            .route(mcp_endpoint, get(handle_mcp_get))
            .route(mcp_endpoint, post(handle_mcp_post))
            .route(mcp_endpoint, delete(handle_mcp_delete))
            .layer(Extension(http_handler));

        router = router
            .route(sse_endpoint, get(handle_sse))
            .route(messages_endpoint, post(handle_messages));

        if let Some(health) = health_endpoint {
            router = router.route(health, get(handle_health));
        }

        router = router.fallback(not_found);

        router.with_state(state).layer(cors)
    }

    pub async fn start(
        state: Arc<McpAppState>,
        http_handler: Arc<McpHttpHandler>,
        options: HyperServerOptions,
    ) -> rust_mcp_sdk::error::SdkResult<()> {
        let addr: SocketAddr = format!("{}:{}", options.host, options.port)
            .parse()
            .map_err(
                |e: std::net::AddrParseError| rust_mcp_sdk::error::McpSdkError::Internal {
                    description: e.to_string(),
                },
            )?;

        let router = Self::build_router(state, http_handler, &options);

        tracing::info!("CORS-enabled server starting on http://{}", addr);
        tracing::info!(
            "MCP endpoint: http://{}{}",
            addr,
            options.streamable_http_endpoint()
        );
        tracing::info!("SSE endpoint: http://{}{}", addr, options.sse_endpoint());

        let handle: axum_server::Handle<SocketAddr> = axum_server::Handle::new();
        let handle_clone = handle.clone();

        tokio::spawn(async move {
            use tokio::signal;

            let ctrl_c = async {
                signal::ctrl_c()
                    .await
                    .expect("Failed to install Ctrl+C handler");
            };

            #[cfg(unix)]
            let terminate = async {
                signal::unix::signal(signal::unix::SignalKind::terminate())
                    .expect("Failed to install signal handler")
                    .recv()
                    .await;
            };

            #[cfg(not(unix))]
            let terminate = std::future::pending::<()>();

            tokio::select! {
                _ = ctrl_c => {},
                _ = terminate => {},
            }

            tracing::info!("Signal received, starting graceful shutdown");
            handle_clone.graceful_shutdown(Some(Duration::from_secs(5)));
        });

        axum_server::bind(addr)
            .handle(handle)
            .serve(router.into_make_service())
            .await
            .map_err(rust_mcp_sdk::error::McpSdkError::Io)?;

        Ok(())
    }
}

pub async fn handle_mcp_get(
    headers: HeaderMap,
    uri: Uri,
    State(state): State<Arc<McpAppState>>,
    Extension(http_handler): Extension<Arc<McpHttpHandler>>,
) -> impl IntoResponse {
    let request = McpHttpHandler::create_request(Method::GET, uri, headers, None);
    match http_handler.handle_streamable_http(request, state).await {
        Ok(generic_res) => {
            let (parts, body) = generic_res.into_parts();
            axum::response::Response::from_parts(parts, axum::body::Body::new(body))
        }
        Err(e) => {
            let status = http::StatusCode::INTERNAL_SERVER_ERROR;
            (status, format!("Error: {}", e)).into_response()
        }
    }
}

pub async fn handle_mcp_post(
    headers: HeaderMap,
    uri: Uri,
    State(state): State<Arc<McpAppState>>,
    Extension(http_handler): Extension<Arc<McpHttpHandler>>,
    Query(_params): Query<HashMap<String, String>>,
    payload: String,
) -> impl IntoResponse {
    let request = McpHttpHandler::create_request(Method::POST, uri, headers, Some(&payload));
    match http_handler.handle_streamable_http(request, state).await {
        Ok(generic_res) => {
            let (parts, body) = generic_res.into_parts();
            axum::response::Response::from_parts(parts, axum::body::Body::new(body))
        }
        Err(e) => {
            let status = http::StatusCode::INTERNAL_SERVER_ERROR;
            (status, format!("Error: {}", e)).into_response()
        }
    }
}

pub async fn handle_mcp_delete(
    headers: HeaderMap,
    uri: Uri,
    State(state): State<Arc<McpAppState>>,
    Extension(http_handler): Extension<Arc<McpHttpHandler>>,
) -> impl IntoResponse {
    let request = McpHttpHandler::create_request(Method::DELETE, uri, headers, None);
    match http_handler.handle_streamable_http(request, state).await {
        Ok(generic_res) => {
            let (parts, body) = generic_res.into_parts();
            axum::response::Response::from_parts(parts, axum::body::Body::new(body))
        }
        Err(e) => {
            let status = http::StatusCode::INTERNAL_SERVER_ERROR;
            (status, format!("Error: {}", e)).into_response()
        }
    }
}

pub async fn handle_sse(
    headers: HeaderMap,
    uri: Uri,
    State(state): State<Arc<McpAppState>>,
    Extension(http_handler): Extension<Arc<McpHttpHandler>>,
) -> impl IntoResponse {
    let request = McpHttpHandler::create_request(Method::GET, uri, headers, None);
    match http_handler
        .handle_sse_connection(request, state, Some("/messages"))
        .await
    {
        Ok(generic_res) => {
            let (parts, body) = generic_res.into_parts();
            axum::response::Response::from_parts(parts, axum::body::Body::new(body))
        }
        Err(e) => {
            let status = http::StatusCode::INTERNAL_SERVER_ERROR;
            (status, format!("Error: {}", e)).into_response()
        }
    }
}

pub async fn handle_messages(
    headers: HeaderMap,
    uri: Uri,
    State(state): State<Arc<McpAppState>>,
    Extension(http_handler): Extension<Arc<McpHttpHandler>>,
    message: String,
) -> impl IntoResponse {
    let request = McpHttpHandler::create_request(Method::POST, uri, headers, Some(&message));
    match http_handler.handle_sse_message(request, state).await {
        Ok(generic_res) => {
            let (parts, body) = generic_res.into_parts();
            axum::response::Response::from_parts(parts, axum::body::Body::new(body))
        }
        Err(e) => {
            let status = http::StatusCode::INTERNAL_SERVER_ERROR;
            (status, format!("Error: {}", e)).into_response()
        }
    }
}

pub async fn handle_health(
    headers: HeaderMap,
    uri: Uri,
    Extension(http_handler): Extension<Arc<McpHttpHandler>>,
) -> impl IntoResponse {
    let request = McpHttpHandler::create_request(Method::GET, uri, headers, None);
    match http_handler.handle_health(request).await {
        Ok(generic_res) => {
            let (parts, body) = generic_res.into_parts();
            axum::response::Response::from_parts(parts, axum::body::Body::new(body))
        }
        Err(e) => {
            let status = http::StatusCode::INTERNAL_SERVER_ERROR;
            (status, format!("Error: {}", e)).into_response()
        }
    }
}

pub async fn not_found(uri: Uri) -> impl IntoResponse {
    (
        http::StatusCode::NOT_FOUND,
        format!("The requested uri does not exist:\r\nuri: {uri}"),
    )
}
