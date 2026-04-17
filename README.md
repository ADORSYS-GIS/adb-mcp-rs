# ADB MCP Server

A high-performance MCP server for Android Debug Bridge operations and E2E testing.

## Architecture

**SOLID Principles + MVP Pattern**:

- **Model**: `src/adb/` - ADB commands, execution, parsing
- **View**: MCP protocol responses (via `rust-mcp-sdk`)
- **Presenter**: `src/tools/` - Tool handlers orchestrating Model→View

**Modular Structure** (max 150 lines/file):

```
src/
├── main.rs          # Entry point
├── cli.rs           # Clap CLI with ENV support
├── adb/
│   ├── command.rs   # Command builder pattern
│   ├── executor.rs  # Command execution
│   └── parser.rs    # Output parsing
├── tools/
│   ├── mod.rs       # ToolContext trait + registry builder
│   ├── traits.rs    # ToolResult, ToolError
│   ├── context.rs   # AdbContext implementation
│   ├── registry.rs  # Tool registry & dispatch
│   ├── device.rs    # Device tools
│   ├── app.rs       # App management tools
│   ├── file.rs      # File operations
│   ├── ui.rs        # UI automation tools
│   ├── media.rs     # Screenshot/recording
│   ├── debug.rs     # Logcat, shell
│   └── network.rs   # Port forwarding, wireless
└── server/
    ├── handler.rs   # ServerHandler impl
    ├── info.rs      # Server info builder
    └── runtime.rs   # Server startup
```

## Features

- **Rust 2024 Edition**
- **clap** CLI with ENV var support (`ADB_MCP_PORT`, `ADB_MCP_LOG_LEVEL`)
- **mimalloc** - High-performance allocator
- **cuid2** - Collision-resistant IDs
- **33 tools** across 7 categories

## Installation

```bash
cargo build --release
```

## Usage

```bash
# stdio mode
./adb-mcp

# HTTP mode
./adb-mcp http -p 8080
ADB_MCP_PORT=3000 ./adb-mcp http

# With debug logging
./adb-mcp -l debug
```

## Environment Variables

| Variable            | Default | Description                             |
|---------------------|---------|-----------------------------------------|
| `ADB_MCP_PORT`      | 8080    | HTTP server port                        |
| `ADB_MCP_LOG_LEVEL` | info    | Log level (trace/debug/info/warn/error) |

## Tool Categories

| Category | Tools                                                                                                                                                                     |
|----------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Device   | `adb_devices`, `adb_device_info`                                                                                                                                          |
| App      | `adb_install`, `adb_uninstall`, `adb_start_app`, `adb_stop_app`, `adb_clear_app`                                                                                          |
| File     | `adb_push`, `adb_pull`                                                                                                                                                    |
| UI       | `adb_tap`, `adb_tap_by_text`, `adb_swipe`, `adb_input_text`, `adb_press_key`, `adb_wait_for_element`, `adb_set_orientation`                                               |
| Media    | `adb_screenshot`, `adb_screenrecord`, `adb_ui_hierarchy`                                                                                                                  |
| Debug    | `adb_shell`, `adb_logcat`                                                                                                                                                 |
| Network  | `adb_forward`, `adb_reverse`, `adb_tcpip`, `adb_usb`, `adb_connect`, `adb_disconnect`, `adb_list_forward`, `adb_list_reverse`, `adb_remove_forward`, `adb_remove_reverse` |

## Dev Utilities Examples

### Metro/React Native

```bash
adb_forward(local_port: 8081, remote_port: 8081)
```

### Flutter DevTools

```bash
adb_forward(local_port: 12345, remote_port: 12345)
```

### API on Host

```bash
adb_reverse(remote_port: 3000, local_port: 3000)
```

### Wireless Debugging

```bash
adb_tcpip()  # Returns: adb connect 192.168.1.x:5555
adb_connect(host: "192.168.1.100")
```

## Claude Desktop Config

```json
{
  "mcpServers": {
    "adb": {
      "command": "/path/to/adb-mcp"
    }
  }
}
```

## Dependencies

- `rust-mcp-sdk` v0.9
- `tokio` - Async runtime
- `clap` - CLI with derive + env features
- `mimalloc` - Memory allocator
- `cuid2` - Collision-resistant IDs
- `thiserror` - Error derive

## License

MIT
