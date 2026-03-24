# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`ngxm` — a Rust CLI tool for managing Nginx and Ngrok services on Linux. Supports both an interactive terminal menu (`inquire`) and direct CLI commands (`clap`).

## Build & Run

```bash
cargo build --release              # Build release
cargo run                          # Interactive menu
cargo run -- <command>             # Direct CLI (e.g. cargo run -- status)
ngxm-build                        # Alias: clean + install to ~/.cargo/bin/
cargo clean -p ngxm                # Clean only this project (keeps deps)
```

Rust edition 2024 (stable since Rust 1.85).

## Architecture

```
src/
  main.rs       — entry point, dispatches CLI args or interactive menu
  cli.rs        — clap argument/subcommand definitions
  commands.rs   — MenuAction enum, NgrokService enum, execute logic, run_command helpers
  menu.rs       — interactive menu with 2-level navigation (MainMenu → sub-menus)
  error.rs      — AppError enum (thiserror), Result type alias
```

- **Dual mode**: no args → interactive menu (`menu::interactive_menu`), with args → direct execution (`execute_cli`)
- **Menu structure**: top-level menu (Nginx / Ngrok / Thoát) → sub-menus per category
- `MenuAction::nginx_actions()` and `MenuAction::ngrok_actions()` define sub-menu items
- `run_command()` captures output (`.output()`), `run_command_streaming()` streams real-time (`.status()`) for long-running commands like `apt install`
- Ngrok URL feature calls `curl http://localhost:{port}/api/tunnels` and parses JSON with `serde_json`

## CLI Commands

```
ngxm install|status|start|restart|reload|stop   # Nginx management
ngxm test-config                                  # nginx -t
ngxm logs access|error                            # tail -n 50 nginx logs
ngxm ngrok|ngrok-start|ngrok-stop|ngrok-restart <gateway|portal>
ngxm ngrok-url <gateway|portal>                   # Get public URL from ngrok API
```

## Notes

- Requires `sudo` privileges at runtime for systemctl/apt commands.
- Ngrok API ports: gateway = 4040, portal = 4041.
- UI text and comments are in Vietnamese.
