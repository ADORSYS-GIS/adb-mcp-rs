use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "adb-mcp")]
#[command(about = "ADB MCP Server for Android device interaction")]
#[command(version)]
pub struct Cli {
    /// Server mode: 'stdio' or 'http'
    #[arg(value_enum, default_value = "stdio")]
    pub mode: Mode,

    /// Port for HTTP mode
    #[arg(short, long, default_value = "8080", env = "ADB_MCP_PORT")]
    pub port: u16,

    /// Log level
    #[arg(short, long, default_value = "info", env = "ADB_MCP_LOG_LEVEL")]
    pub log_level: String,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Mode {
    Stdio,
    Http,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Stdio => write!(f, "stdio"),
            Mode::Http => write!(f, "http"),
        }
    }
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Stdio => "stdio",
            Mode::Http => "http",
        }
    }
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn init_logging(&self) {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::new(&self.log_level))
            .init();
    }
}
