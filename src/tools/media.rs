use super::{ToolContext, ToolError, ToolResult, registry::ToolRegistry};
use rust_mcp_sdk::macros;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::thread;
use std::time::Duration;

#[macros::mcp_tool(
    name = "adb_screenshot",
    title = "Take Screenshot",
    description = "Capture screen and save locally"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct ScreenshotParams {
    #[serde(default)]
    pub device: Option<String>,
    pub local_path: String,
}

#[macros::mcp_tool(
    name = "adb_screenrecord",
    title = "Record Screen",
    description = "Record screen video (max 180s)"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct ScreenrecordParams {
    #[serde(default)]
    pub device: Option<String>,
    pub local_path: String,
    pub duration_seconds: u32,
    #[serde(default)]
    pub bit_rate: Option<u32>,
}

#[macros::mcp_tool(
    name = "adb_ui_hierarchy",
    title = "Get UI Hierarchy",
    description = "Dump UI hierarchy as XML"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct UiHierarchyParams {
    #[serde(default)]
    pub device: Option<String>,
}

pub fn register(registry: &mut ToolRegistry) {
    registry.register(ScreenshotParams::tool(), handle_screenshot);
    registry.register(ScreenrecordParams::tool(), handle_screenrecord);
    registry.register(UiHierarchyParams::tool(), handle_ui_hierarchy);
}

fn handle_screenshot(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: ScreenshotParams = serde_json::from_value(serde_json::Value::Object(args))?;

    ctx.execute_shell("screencap -p /sdcard/screenshot_temp.png")?;
    ctx.execute_adb(vec![
        "pull",
        "/sdcard/screenshot_temp.png",
        &params.local_path,
    ])?;
    ctx.execute_shell("rm /sdcard/screenshot_temp.png")?;

    Ok(ToolResult::ok(format!(
        "Screenshot saved: {}",
        params.local_path
    )))
}

fn handle_screenrecord(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: ScreenrecordParams = serde_json::from_value(serde_json::Value::Object(args))?;

    if params.duration_seconds > 180 {
        return Err(ToolError::InvalidArgument(
            "Duration cannot exceed 180 seconds".into(),
        ));
    }

    let mut record_cmd = format!(
        "screenrecord --time-limit {} /sdcard/record_temp.mp4",
        params.duration_seconds
    );
    if let Some(bitrate) = params.bit_rate {
        record_cmd.push_str(&format!(" --bit-rate {}", bitrate));
    }

    ctx.execute_shell(&record_cmd)?;
    thread::sleep(Duration::from_secs(params.duration_seconds as u64 + 1));

    ctx.execute_adb(vec!["pull", "/sdcard/record_temp.mp4", &params.local_path])?;
    ctx.execute_shell("rm /sdcard/record_temp.mp4")?;

    Ok(ToolResult::ok(format!(
        "Recording saved: {}",
        params.local_path
    )))
}

fn handle_ui_hierarchy(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let _params: UiHierarchyParams = serde_json::from_value(serde_json::Value::Object(args))?;

    ctx.execute_shell("uiautomator dump /sdcard/ui_hierarchy.xml")?;
    let xml = ctx.execute_shell("cat /sdcard/ui_hierarchy.xml")?;
    ctx.execute_shell("rm /sdcard/ui_hierarchy.xml")?;

    Ok(ToolResult::ok(xml))
}
