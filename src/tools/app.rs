use super::{ToolContext, ToolError, ToolResult, registry::ToolRegistry};
use rust_mcp_sdk::macros;
use serde::{Deserialize, Serialize};
use serde_json::Map;

#[macros::mcp_tool(
    name = "adb_install",
    title = "Install APK",
    description = "Install an APK file on device"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct InstallParams {
    #[serde(default)]
    pub device: Option<String>,
    pub apk_path: String,
    #[serde(default)]
    pub reinstall: bool,
    #[serde(default)]
    pub grant_permissions: bool,
}

#[macros::mcp_tool(
    name = "adb_uninstall",
    title = "Uninstall App",
    description = "Uninstall app by package name"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct UninstallParams {
    #[serde(default)]
    pub device: Option<String>,
    pub package: String,
    #[serde(default)]
    pub keep_data: bool,
}

#[macros::mcp_tool(
    name = "adb_start_app",
    title = "Start App",
    description = "Start an app by package name"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct StartAppParams {
    #[serde(default)]
    pub device: Option<String>,
    pub package: String,
    pub activity: Option<String>,
}

#[macros::mcp_tool(
    name = "adb_stop_app",
    title = "Stop App",
    description = "Force stop an app"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct StopAppParams {
    #[serde(default)]
    pub device: Option<String>,
    pub package: String,
}

#[macros::mcp_tool(
    name = "adb_clear_app",
    title = "Clear App Data",
    description = "Clear all data for an app"
)]
#[derive(Debug, Deserialize, Serialize, macros::JsonSchema)]
pub struct ClearAppParams {
    #[serde(default)]
    pub device: Option<String>,
    pub package: String,
}

pub fn register(registry: &mut ToolRegistry) {
    registry.register(InstallParams::tool(), handle_install);
    registry.register(UninstallParams::tool(), handle_uninstall);
    registry.register(StartAppParams::tool(), handle_start_app);
    registry.register(StopAppParams::tool(), handle_stop_app);
    registry.register(ClearAppParams::tool(), handle_clear_app);
}

fn handle_install(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: InstallParams = serde_json::from_value(serde_json::Value::Object(args))?;

    let mut adb_args = vec!["install"];
    if params.reinstall {
        adb_args.push("-r");
    }
    if params.grant_permissions {
        adb_args.push("-g");
    }
    adb_args.push(&params.apk_path);

    let output = ctx.execute_adb(adb_args)?;
    Ok(ToolResult::ok(format!("Install result: {}", output)))
}

fn handle_uninstall(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: UninstallParams = serde_json::from_value(serde_json::Value::Object(args))?;

    let mut adb_args = vec!["uninstall"];
    if params.keep_data {
        adb_args.push("-k");
    }
    adb_args.push(&params.package);

    let output = ctx.execute_adb(adb_args)?;
    Ok(ToolResult::ok(format!("Uninstall result: {}", output)))
}

fn handle_start_app(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: StartAppParams = serde_json::from_value(serde_json::Value::Object(args))?;

    let result = if let Some(ref act) = params.activity {
        ctx.execute_shell(&format!("am start -n {}/{}", params.package, act))
    } else {
        ctx.execute_shell(&format!(
            "monkey -p {} -c android.intent.category.LAUNCHER 1",
            params.package
        ))
    }?;

    Ok(ToolResult::ok(format!(
        "Started {}: {}",
        params.package, result
    )))
}

fn handle_stop_app(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: StopAppParams = serde_json::from_value(serde_json::Value::Object(args))?;
    ctx.execute_shell(&format!("am force-stop {}", params.package))?;
    Ok(ToolResult::ok(format!("Stopped {}", params.package)))
}

fn handle_clear_app(
    args: Map<String, serde_json::Value>,
    ctx: &dyn ToolContext,
) -> Result<ToolResult, ToolError> {
    let params: ClearAppParams = serde_json::from_value(serde_json::Value::Object(args))?;
    let result = ctx.execute_shell(&format!("pm clear {}", params.package))?;
    Ok(ToolResult::ok(format!(
        "Cleared {}: {}",
        params.package, result
    )))
}
