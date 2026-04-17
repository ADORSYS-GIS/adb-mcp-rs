use super::{ToolContext, ToolError, ToolResult, registry::ToolRegistry};
use crate::adb::parser;
use rust_mcp_sdk::macros;
use serde::{Deserialize, Serialize};
use serde_json::Map;

#[macros::mcp_tool(
    name = "adb_forward",
    title = "Port Forward",
    description = "Forward host port to device port"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct ForwardParams {
    #[serde(default)]
    pub device: Option<String>,
    pub local_port: u16,
    pub remote_port: u16,
    #[serde(default)]
    pub remove_existing: bool,
}

#[macros::mcp_tool(
    name = "adb_reverse",
    title = "Reverse Port",
    description = "Reverse forward device port to host"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct ReverseParams {
    #[serde(default)]
    pub device: Option<String>,
    pub remote_port: u16,
    pub local_port: u16,
    #[serde(default)]
    pub remove_existing: bool,
}

#[macros::mcp_tool(
    name = "adb_tcpip",
    title = "Enable TCP/IP",
    description = "Enable wireless debugging"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct TcpipParams {
    #[serde(default)]
    pub device: Option<String>,
    pub port: Option<u16>,
}

#[macros::mcp_tool(
    name = "adb_usb",
    title = "Switch to USB",
    description = "Switch back to USB mode"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct UsbParams {
    #[serde(default)]
    pub device: Option<String>,
}

#[macros::mcp_tool(
    name = "adb_connect",
    title = "Connect",
    description = "Connect to wireless device"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct ConnectParams {
    pub host: String,
    pub port: Option<u16>,
}

#[macros::mcp_tool(
    name = "adb_disconnect",
    title = "Disconnect",
    description = "Disconnect wireless device(s)"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct DisconnectParams {
    pub host: Option<String>,
    pub port: Option<u16>,
    #[serde(default)]
    pub disconnect_all: bool,
}

#[macros::mcp_tool(
    name = "adb_list_forward",
    title = "List Forwards",
    description = "List port forwards"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct ListForwardParams {
    #[serde(default)]
    pub device: Option<String>,
}

#[macros::mcp_tool(
    name = "adb_list_reverse",
    title = "List Reverse",
    description = "List reverse forwards"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct ListReverseParams {
    #[serde(default)]
    pub device: Option<String>,
}

#[macros::mcp_tool(
    name = "adb_remove_forward",
    title = "Remove Forward",
    description = "Remove port forward"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct RemoveForwardParams {
    #[serde(default)]
    pub device: Option<String>,
    pub local_port: Option<u16>,
    #[serde(default)]
    pub remove_all: bool,
}

#[macros::mcp_tool(
    name = "adb_remove_reverse",
    title = "Remove Reverse",
    description = "Remove reverse forward"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct RemoveReverseParams {
    #[serde(default)]
    pub device: Option<String>,
    pub remote_port: Option<u16>,
    #[serde(default)]
    pub remove_all: bool,
}

pub fn register(registry: &mut ToolRegistry) {
    registry.register(ForwardParams::tool(), handle_forward);
    registry.register(ReverseParams::tool(), handle_reverse);
    registry.register(TcpipParams::tool(), handle_tcpip);
    registry.register(UsbParams::tool(), handle_usb);
    registry.register(ConnectParams::tool(), handle_connect);
    registry.register(DisconnectParams::tool(), handle_disconnect);
    registry.register(ListForwardParams::tool(), handle_list_forward);
    registry.register(ListReverseParams::tool(), handle_list_reverse);
    registry.register(RemoveForwardParams::tool(), handle_remove_forward);
    registry.register(RemoveReverseParams::tool(), handle_remove_reverse);
}

fn handle_forward(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: ForwardParams = serde_json::from_value(serde_json::Value::Object(args))?;

    if params.remove_existing {
        let _ = ctx.execute_adb(vec![
            "forward",
            "--remove",
            &format!("tcp:{}", params.local_port),
        ]);
    }

    ctx.execute_adb(vec![
        "forward",
        &format!("tcp:{}", params.local_port),
        &format!("tcp:{}", params.remote_port),
    ])?;

    Ok(ToolResult::ok(format!(
        "Forward: {} → {}",
        params.local_port, params.remote_port
    )))
}

fn handle_reverse(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: ReverseParams = serde_json::from_value(serde_json::Value::Object(args))?;

    if params.remove_existing {
        let _ = ctx.execute_adb(vec![
            "reverse",
            "--remove",
            &format!("tcp:{}", params.remote_port),
        ]);
    }

    ctx.execute_adb(vec![
        "reverse",
        &format!("tcp:{}", params.remote_port),
        &format!("tcp:{}", params.local_port),
    ])?;

    Ok(ToolResult::ok(format!(
        "Reverse: {} → {}",
        params.remote_port, params.local_port
    )))
}

fn handle_tcpip(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: TcpipParams = serde_json::from_value(serde_json::Value::Object(args))?;
    let port = params.port.unwrap_or(5555);

    ctx.execute_adb(vec!["tcpip", &port.to_string()])?;

    let ip_out =
        ctx.execute_shell("ip addr show wlan0 | grep 'inet ' | awk '{print $2}' | cut -d/ -f1")?;
    let ip = parser::parse_ip_address(&ip_out).unwrap_or_else(|| "<unknown>".into());

    Ok(ToolResult::ok(format!(
        "TCP/IP enabled. Connect: adb connect {}:{}",
        ip, port
    )))
}

fn handle_usb(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: UsbParams = serde_json::from_value(serde_json::Value::Object(args))?;
    ctx.execute_adb(vec!["usb"])?;
    Ok(ToolResult::ok("Switched to USB mode"))
}

fn handle_connect(
    args: Map<String, serde_json::Value>,
    _ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: ConnectParams = serde_json::from_value(serde_json::Value::Object(args))?;
    let port = params.port.unwrap_or(5555);
    let addr = format!("{}:{}", params.host, port);

    let output = std::process::Command::new("adb")
        .args(["connect", &addr])
        .output()
        .map_err(|e| ToolError::Other(e.to_string()))?;

    Ok(ToolResult::ok(
        String::from_utf8_lossy(&output.stdout).trim().to_string(),
    ))
}

fn handle_disconnect(
    args: Map<String, serde_json::Value>,
    _ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: DisconnectParams = serde_json::from_value(serde_json::Value::Object(args))?;

    let output = if params.disconnect_all {
        std::process::Command::new("adb")
            .args(["disconnect"])
            .output()
            .map_err(|e| ToolError::Other(e.to_string()))?
    } else if let Some(ref host) = params.host {
        let port = params.port.unwrap_or(5555);
        std::process::Command::new("adb")
            .args(["disconnect", &format!("{}:{}", host, port)])
            .output()
            .map_err(|e| ToolError::Other(e.to_string()))?
    } else {
        return Err(ToolError::InvalidArgument(
            "Specify host or use disconnect_all".into(),
        ));
    };

    Ok(ToolResult::ok(
        String::from_utf8_lossy(&output.stdout).trim().to_string(),
    ))
}

fn handle_list_forward(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: ListForwardParams = serde_json::from_value(serde_json::Value::Object(args))?;
    let output = ctx.execute_adb(vec!["forward", "--list"])?;
    Ok(ToolResult::ok(output))
}

fn handle_list_reverse(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: ListReverseParams = serde_json::from_value(serde_json::Value::Object(args))?;
    let output = ctx.execute_adb(vec!["reverse", "--list"])?;
    Ok(ToolResult::ok(output))
}

fn handle_remove_forward(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: RemoveForwardParams = serde_json::from_value(serde_json::Value::Object(args))?;

    if params.remove_all {
        ctx.execute_adb(vec!["forward", "--remove-all"])?;
    } else if let Some(port) = params.local_port {
        ctx.execute_adb(vec!["forward", "--remove", &format!("tcp:{}", port)])?;
    } else {
        return Err(ToolError::InvalidArgument(
            "Specify local_port or use remove_all".into(),
        ));
    }

    Ok(ToolResult::ok("Forward removed"))
}

fn handle_remove_reverse(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: RemoveReverseParams = serde_json::from_value(serde_json::Value::Object(args))?;

    if params.remove_all {
        ctx.execute_adb(vec!["reverse", "--remove-all"])?;
    } else if let Some(port) = params.remote_port {
        ctx.execute_adb(vec!["reverse", "--remove", &format!("tcp:{}", port)])?;
    } else {
        return Err(ToolError::InvalidArgument(
            "Specify remote_port or use remove_all".into(),
        ));
    }

    Ok(ToolResult::ok("Reverse forward removed"))
}
