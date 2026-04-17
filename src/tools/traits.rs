use rust_mcp_sdk::macros::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct DeviceRef {
    #[serde(default)]
    pub device: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ToolResult {
    pub content: String,
    pub is_error: bool,
}

impl ToolResult {
    pub fn ok(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_error: false,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            content: message.into(),
            is_error: true,
        }
    }
}

impl From<ToolResult> for String {
    fn from(result: ToolResult) -> String {
        result.content
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("ADB error: {0}")]
    Adb(#[from] crate::adb::AdbError),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Element not found")]
    ElementNotFound,

    #[error("Timeout waiting for element")]
    Timeout,

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("{0}")]
    Other(String),
}

impl From<serde_json::Error> for ToolError {
    fn from(e: serde_json::Error) -> Self {
        ToolError::Parse(e.to_string())
    }
}
