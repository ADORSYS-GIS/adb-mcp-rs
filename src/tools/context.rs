use super::{ToolContext, ToolError};
use crate::adb::{AdbCommand, AdbExecutor, CommandExecutor};
use std::sync::Arc;

#[derive(Clone)]
pub struct AdbContext {
    device: Option<String>,
    executor: Arc<AdbExecutor>,
}

impl AdbContext {
    pub fn new(device: Option<String>) -> Self {
        Self {
            device,
            executor: Arc::new(AdbExecutor::new()),
        }
    }

    pub fn shell_cmd(&self, cmd: &str) -> AdbCommand {
        let mut command = AdbCommand::shell(cmd);
        if let Some(ref d) = self.device {
            command = command.with_device(d);
        }
        command
    }
}

impl ToolContext for AdbContext {
    fn execute_shell(&self, cmd: &str) -> Result<String, ToolError> {
        let command = self.shell_cmd(cmd);
        let output = self.executor.execute(command)?;
        output.into_result().map_err(Into::into)
    }

    fn execute_adb(&self, args: Vec<&str>) -> Result<String, ToolError> {
        let mut cmd = std::process::Command::new("adb");
        if let Some(ref d) = self.device {
            cmd.args(["-s", d]);
        }
        cmd.args(&args);
        let output = cmd.output().map_err(|e| ToolError::Other(e.to_string()))?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        if output.status.success() {
            Ok(stdout)
        } else {
            Err(ToolError::Other(
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ))
        }
    }
}
