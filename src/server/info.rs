use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, ProtocolVersion, ServerCapabilities, ServerCapabilitiesTools,
};

pub struct ServerInfoBuilder {
    name: String,
    version: String,
    title: String,
    description: String,
    instructions: String,
}

impl ServerInfoBuilder {
    pub fn new() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            title: "ADB MCP Server".to_string(),
            description: "MCP server for Android Debug Bridge operations".to_string(),
            instructions: String::new(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = instructions.into();
        self
    }

    pub fn build(self) -> InitializeResult {
        InitializeResult {
            server_info: Implementation {
                name: self.name,
                version: self.version,
                title: Some(self.title),
                description: Some(self.description),
                icons: vec![],
                website_url: None,
            },
            capabilities: ServerCapabilities {
                tools: Some(ServerCapabilitiesTools { list_changed: None }),
                ..Default::default()
            },
            protocol_version: ProtocolVersion::V2025_11_25.into(),
            instructions: if self.instructions.is_empty() {
                Some(Self::default_instructions())
            } else {
                Some(self.instructions)
            },
            meta: None,
        }
    }

    fn default_instructions() -> String {
        "ADB MCP Server - 33 tools for Android device interaction.\n\
         Categories: Device, App, File, UI, Media, Debug, Network utilities.\n\
         Use adb_forward for Metro/DevTools, adb_reverse for API servers."
            .to_string()
    }
}

impl Default for ServerInfoBuilder {
    fn default() -> Self {
        Self::new()
    }
}
