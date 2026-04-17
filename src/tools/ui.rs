use super::{ToolContext, ToolError, ToolResult, registry::ToolRegistry};
use crate::adb::parser;
use rust_mcp_sdk::macros;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::thread;
use std::time::Duration;

#[macros::mcp_tool(
    name = "adb_tap",
    title = "Tap at Coordinates",
    description = "Tap at screen coordinates"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct TapParams {
    #[serde(default)]
    pub device: Option<String>,
    pub x: u32,
    pub y: u32,
}

#[macros::mcp_tool(
    name = "adb_tap_by_text",
    title = "Tap by Text/ID",
    description = "Find and tap UI element"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct TapByTextParams {
    #[serde(default)]
    pub device: Option<String>,
    pub text: Option<String>,
    pub resource_id: Option<String>,
    pub content_desc: Option<String>,
    #[serde(default)]
    pub index: Option<u32>,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

#[macros::mcp_tool(
    name = "adb_swipe",
    title = "Swipe Gesture",
    description = "Perform swipe gesture"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct SwipeParams {
    #[serde(default)]
    pub device: Option<String>,
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
    #[serde(default)]
    pub duration_ms: Option<u32>,
}

#[macros::mcp_tool(
    name = "adb_input_text",
    title = "Input Text",
    description = "Type text into focused field"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct InputTextParams {
    #[serde(default)]
    pub device: Option<String>,
    pub text: String,
}

#[macros::mcp_tool(
    name = "adb_press_key",
    title = "Press Key",
    description = "Press a key (HOME, BACK, etc.)"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct PressKeyParams {
    #[serde(default)]
    pub device: Option<String>,
    pub key: String,
}

#[macros::mcp_tool(
    name = "adb_wait_for_element",
    title = "Wait for Element",
    description = "Wait for UI element"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct WaitForElementParams {
    #[serde(default)]
    pub device: Option<String>,
    pub text: Option<String>,
    pub resource_id: Option<String>,
    pub content_desc: Option<String>,
    #[serde(default)]
    pub timeout_ms: u64,
}

#[macros::mcp_tool(
    name = "adb_set_orientation",
    title = "Set Orientation",
    description = "Set screen orientation"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct SetOrientationParams {
    #[serde(default)]
    pub device: Option<String>,
    pub orientation: String,
}

pub fn register(registry: &mut ToolRegistry) {
    registry.register(TapParams::tool(), handle_tap);
    registry.register(TapByTextParams::tool(), handle_tap_by_text);
    registry.register(SwipeParams::tool(), handle_swipe);
    registry.register(InputTextParams::tool(), handle_input_text);
    registry.register(PressKeyParams::tool(), handle_press_key);
    registry.register(WaitForElementParams::tool(), handle_wait_for_element);
    registry.register(SetOrientationParams::tool(), handle_set_orientation);
}

fn handle_tap(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: TapParams = serde_json::from_value(serde_json::Value::Object(args))?;
    ctx.execute_shell(&format!("input tap {} {}", params.x, params.y))?;
    Ok(ToolResult::ok(format!(
        "Tapped at ({}, {})",
        params.x, params.y
    )))
}

fn handle_tap_by_text(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: TapByTextParams = serde_json::from_value(serde_json::Value::Object(args))?;
    let timeout = params.timeout_ms.unwrap_or(5000);

    let coords = find_element(
        ctx,
        params.text.as_deref(),
        params.resource_id.as_deref(),
        params.content_desc.as_deref(),
        params.index.map(|i| i as usize).unwrap_or(0),
        timeout,
    )?;

    ctx.execute_shell(&format!("input tap {} {}", coords.0, coords.1))?;
    Ok(ToolResult::ok(format!(
        "Tapped element at ({}, {})",
        coords.0, coords.1
    )))
}

fn handle_swipe(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: SwipeParams = serde_json::from_value(serde_json::Value::Object(args))?;

    let cmd = match params.duration_ms {
        Some(d) => format!(
            "input swipe {} {} {} {} {}",
            params.x1, params.y1, params.x2, params.y2, d
        ),
        None => format!(
            "input swipe {} {} {} {}",
            params.x1, params.y1, params.x2, params.y2
        ),
    };
    ctx.execute_shell(&cmd)?;
    Ok(ToolResult::ok(format!(
        "Swiped from ({}, {}) to ({}, {})",
        params.x1, params.y1, params.x2, params.y2
    )))
}

fn handle_input_text(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: InputTextParams = serde_json::from_value(serde_json::Value::Object(args))?;
    let escaped = params.text.replace(' ', "%s");
    ctx.execute_shell(&format!("input text {}", escaped))?;
    Ok(ToolResult::ok(format!("Input: {}", params.text)))
}

fn handle_press_key(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: PressKeyParams = serde_json::from_value(serde_json::Value::Object(args))?;
    let key_code = resolve_key_code(&params.key);
    ctx.execute_shell(&format!("input keyevent {}", key_code))?;
    Ok(ToolResult::ok(format!("Pressed: {}", params.key)))
}

fn handle_wait_for_element(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: WaitForElementParams = serde_json::from_value(serde_json::Value::Object(args))?;
    let start = std::time::Instant::now();

    loop {
        match find_element(
            ctx,
            params.text.as_deref(),
            params.resource_id.as_deref(),
            params.content_desc.as_deref(),
            0,
            1000,
        ) {
            Ok(coords) => {
                return Ok(ToolResult::ok(format!(
                    "Element found at ({}, {})",
                    coords.0, coords.1
                )));
            }
            Err(_) => {
                if start.elapsed().as_millis() as u64 > params.timeout_ms {
                    return Err(ToolError::Timeout);
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

fn handle_set_orientation(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: SetOrientationParams = serde_json::from_value(serde_json::Value::Object(args))?;

    let cmd = match params.orientation.to_lowercase().as_str() {
        "portrait" => {
            "settings put system accelerometer_rotation 0 && settings put system user_rotation 0"
        }
        "landscape" => {
            "settings put system accelerometer_rotation 0 && settings put system user_rotation 1"
        }
        "auto" => "settings put system accelerometer_rotation 1",
        _ => {
            return Err(ToolError::InvalidArgument(
                "Use: portrait, landscape, or auto".into(),
            ));
        }
    };

    ctx.execute_shell(cmd)?;
    Ok(ToolResult::ok(format!(
        "Orientation: {}",
        params.orientation
    )))
}

fn resolve_key_code(key: &str) -> String {
    let upper = key.to_uppercase();
    match upper.as_str() {
        "HOME" => "KEYCODE_HOME".into(),
        "BACK" => "KEYCODE_BACK".into(),
        "MENU" => "KEYCODE_MENU".into(),
        "POWER" => "KEYCODE_POWER".into(),
        "VOLUME_UP" => "KEYCODE_VOLUME_UP".into(),
        "VOLUME_DOWN" => "KEYCODE_VOLUME_DOWN".into(),
        "ENTER" => "KEYCODE_ENTER".into(),
        "TAB" => "KEYCODE_TAB".into(),
        "DPAD_UP" => "KEYCODE_DPAD_UP".into(),
        "DPAD_DOWN" => "KEYCODE_DPAD_DOWN".into(),
        "DPAD_LEFT" => "KEYCODE_DPAD_LEFT".into(),
        "DPAD_RIGHT" => "KEYCODE_DPAD_RIGHT".into(),
        "APP_SWITCH" | "RECENT_APPS" => "KEYCODE_APP_SWITCH".into(),
        k if k.starts_with("KEYCODE_") => k.into(),
        k => format!("KEYCODE_{}", k),
    }
}

fn find_element(
    ctx: &dyn ToolContext,
    text: Option<&str>,
    resource_id: Option<&str>,
    content_desc: Option<&str>,
    index: usize,
    _timeout_ms: u64,
) -> Result<(u32, u32), ToolError> {
    ctx.execute_shell("uiautomator dump /sdcard/ui_temp.xml")?;
    let xml = ctx.execute_shell("cat /sdcard/ui_temp.xml")?;
    ctx.execute_shell("rm /sdcard/ui_temp.xml")?;

    parser::find_element_in_xml(&xml, text, resource_id, content_desc, index)
        .ok_or(ToolError::ElementNotFound)
}
