use super::{ToolContext, ToolError, ToolResult, registry::ToolRegistry};
use rust_mcp_sdk::macros;
use serde::{Deserialize, Serialize};
use serde_json::Map;

#[macros::mcp_tool(
    name = "adb_shell",
    title = "Execute Shell",
    description = "Run shell command on device"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct ShellParams {
    #[serde(default)]
    pub device: Option<String>,
    pub command: String,
}

#[macros::mcp_tool(
    name = "adb_logcat",
    title = "View Logcat",
    description = "View device logs"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct LogcatParams {
    #[serde(default)]
    pub device: Option<String>,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub package: Option<String>,
    #[serde(default)]
    pub lines: Option<u32>,
    #[serde(default)]
    pub clear: bool,
}

pub fn register(registry: &mut ToolRegistry) {
    registry.register(ShellParams::tool(), handle_shell);
    registry.register(LogcatParams::tool(), handle_logcat);
}

fn handle_shell(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: ShellParams = serde_json::from_value(serde_json::Value::Object(args))?;
    let output = ctx.execute_shell(&params.command)?;
    Ok(ToolResult::ok(output))
}

fn handle_logcat(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: LogcatParams = serde_json::from_value(serde_json::Value::Object(args))?;

    if params.clear {
        ctx.execute_shell("logcat -c")?;
        return Ok(ToolResult::ok("Logcat cleared"));
    }

    let mut cmd = String::from("logcat -d");

    if let Some(lines) = params.lines {
        cmd.push_str(&format!(" -t {}", lines));
    }

    if let Some(ref pkg) = params.package {
        let pid = ctx.execute_shell(&format!("pidof {}", pkg))?;
        let pid = pid.trim();
        if !pid.is_empty() {
            cmd.push_str(&format!(" --pid={}", pid));
        }
    } else if let Some(ref tag) = params.tag {
        let priority = params.priority.as_deref().unwrap_or("*");
        cmd.push_str(&format!(" {}:{}", tag, priority));
    }

    let output = ctx.execute_shell(&cmd)?;
    Ok(ToolResult::ok(output))
}
