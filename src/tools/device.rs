use super::{ToolContext, ToolError, ToolResult, registry::ToolRegistry};
use crate::adb::parser;
use rust_mcp_sdk::macros;
use serde::{Deserialize, Serialize};
use serde_json::Map;

#[macros::mcp_tool(
    name = "adb_devices",
    title = "List ADB Devices",
    description = "List all connected Android devices"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct DevicesParams {}

#[macros::mcp_tool(
    name = "adb_device_info",
    title = "Get Device Info",
    description = "Get detailed device information"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct DeviceInfoParams {
    #[serde(default)]
    pub device: Option<String>,
}

pub fn register(registry: &mut ToolRegistry) {
    registry.register(DevicesParams::tool(), handle_devices);
    registry.register(DeviceInfoParams::tool(), handle_device_info);
}

fn handle_devices(
    _args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let output = ctx.execute_adb(vec!["devices", "-l"])?;
    let devices = parser::parse_devices(&output);

    let mut result = String::from("Connected devices:\n");
    for d in devices {
        result.push_str(&format!("  {} ({})", d.serial, d.state));
        if let Some(m) = d.model {
            result.push_str(&format!(" - {}", m));
        }
        result.push('\n');
    }

    Ok(ToolResult::ok(result))
}

fn handle_device_info(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: DeviceInfoParams = serde_json::from_value(serde_json::Value::Object(args))?;
    // Verify device exists if specified
    if let Some(ref d) = params.device {
        ctx.execute_adb(vec!["-s", d, "get-state"])?;
    }

    let mut info = String::new();
    info.push_str("=== Device Info ===\n");

    let props = [
        ("Model", "getprop ro.product.model"),
        ("Brand", "getprop ro.product.brand"),
        ("Device", "getprop ro.product.device"),
        ("Android", "getprop ro.build.version.release"),
        ("SDK", "getprop ro.build.version.sdk"),
        ("Build", "getprop ro.build.display.id"),
    ];

    for (label, cmd) in props {
        match ctx.execute_shell(cmd) {
            Ok(val) => info.push_str(&format!("{}: {}\n", label, val.trim())),
            Err(_) => info.push_str(&format!("{}: <unknown>\n", label)),
        }
    }

    if let Ok(density) = ctx.execute_shell("wm density") {
        info.push_str(&format!("Density: {}\n", density.trim()));
    }
    if let Ok(size) = ctx.execute_shell("wm size") {
        info.push_str(&format!("Screen: {}\n", size.trim()));
    }

    Ok(ToolResult::ok(info))
}
