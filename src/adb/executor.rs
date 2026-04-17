use super::command::AdbCommand;
use std::process::Output;

#[derive(Debug, Clone)]
pub struct AdbOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

pub trait CommandExecutor {
    type Error: std::fmt::Display;

    fn execute(&self, command: AdbCommand) -> Result<AdbOutput, Self::Error>;
}

#[derive(Debug, Default)]
pub struct AdbExecutor;

impl AdbExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl CommandExecutor for AdbExecutor {
    type Error = AdbError;

    fn execute(&self, command: AdbCommand) -> Result<AdbOutput, Self::Error> {
        let output = command
            .build()
            .output()
            .map_err(|e| AdbError::ExecutionFailed(e.to_string()))?;

        Ok(AdbOutput::from(output))
    }
}

impl From<Output> for AdbOutput {
    fn from(output: Output) -> Self {
        Self {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            success: output.status.success(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AdbError {
    #[error("ADB execution failed: {0}")]
    ExecutionFailed(String),

    #[error("ADB command failed: {0}")]
    CommandFailed(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}

impl AdbOutput {
    pub fn into_result(self) -> Result<String, AdbError> {
        if self.success || self.stdout.contains("Success") {
            Ok(self.stdout)
        } else {
            Err(AdbError::CommandFailed(if self.stderr.is_empty() {
                self.stdout
            } else {
                self.stderr
            }))
        }
    }
}
