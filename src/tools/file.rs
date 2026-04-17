use super::{ToolContext, ToolError, ToolResult, registry::ToolRegistry};
use rust_mcp_sdk::macros;
use serde::{Deserialize, Serialize};
use serde_json::Map;

#[macros::mcp_tool(
    name = "adb_push",
    title = "Push File",
    description = "Push file/directory to device"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct PushParams {
    #[serde(default)]
    pub device: Option<String>,
    pub local_path: String,
    pub remote_path: String,
}

#[macros::mcp_tool(
    name = "adb_pull",
    title = "Pull File",
    description = "Pull file/directory from device"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct PullParams {
    #[serde(default)]
    pub device: Option<String>,
    pub remote_path: String,
    pub local_path: String,
}

pub fn register(registry: &mut ToolRegistry) {
    registry.register(PushParams::tool(), handle_push);
    registry.register(PullParams::tool(), handle_pull);
}

fn handle_push(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: PushParams = serde_json::from_value(serde_json::Value::Object(args))?;

    let output = ctx.execute_adb(vec!["push", &params.local_path, &params.remote_path])?;

    Ok(ToolResult::ok(output))
}

fn handle_pull(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: PullParams = serde_json::from_value(serde_json::Value::Object(args))?;

    let output = ctx.execute_adb(vec!["pull", &params.remote_path, &params.local_path])?;

    Ok(ToolResult::ok(output))
}
