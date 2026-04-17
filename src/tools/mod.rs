mod app;
mod context;
mod debug;
mod device;
mod file;
mod media;
mod network;
mod registry;
mod traits;
mod ui;

pub use context::AdbContext;
pub use registry::ToolRegistry;
pub use traits::{ToolError, ToolResult};

pub fn build_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();

    device::register(&mut registry);
    app::register(&mut registry);
    file::register(&mut registry);
    ui::register(&mut registry);
    media::register(&mut registry);
    debug::register(&mut registry);
    network::register(&mut registry);

    registry
}

pub trait ToolContext: Send + Sync {
    fn execute_shell(&self, cmd: &str) -> Result<String, ToolError>;
    fn execute_adb(&self, args: Vec<&str>) -> Result<String, ToolError>;
}
