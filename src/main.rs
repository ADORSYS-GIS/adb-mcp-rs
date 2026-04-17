use mimalloc::MiMalloc;

mod adb;
mod cli;
mod server;
mod tools;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> rust_mcp_sdk::error::SdkResult<()> {
    let cli = cli::Cli::parse_args();
    cli.init_logging();

    server::ServerRuntime::start(&cli).await
}
