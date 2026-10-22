# AGENTS.md - Discord Bot Template (Rust)

## Build/Lint/Test Commands

### Development

- `cargo run` - Run bot in development mode
- `cargo build` - Compile in debug mode
- `cargo build --release` - Compile optimized binary to `./target/release/`
- `RUST_LOG=debug cargo run` - Run with verbose console logs

### Linting & Formatting

- `cargo fmt` - Format code (config in `rustfmt.toml`)
- `cargo fmt --check` - Check formatting without changing files
- `cargo clippy --all-targets -- -D warnings` - Lint (warnings fail CI)

### Testing

- `cargo test` - Run all tests
- `cargo test <name>` - Run a single test by name

Unit tests live next to the code in `#[cfg(test)] mod tests`.

## Code Style Guidelines

### Imports

Order (separated by blank lines, `rustfmt` keeps each group sorted):

1. `std`
2. External crates (`poise`, `tracing`, ...)
3. Internal modules (`crate::...`)

Use `use poise::serenity_prelude as serenity;` to namespace serenity types.

### Naming Conventions

- **Files/Modules/Functions/Variables**: `snake_case` (e.g., `set_status.rs`, `is_bot_owner`)
- **Types/Enums/Traits**: `PascalCase` (e.g., `Config`, `Data`, `Status`)
- **Constants**: `UPPER_SNAKE_CASE`
- Slash command names in Discord use kebab-case via `rename = "set-status"`

### Comments

- **Never write comments in code**. Code must be 100% self-explanatory through clean structure and descriptive names.

### Error Handling

- Command and event functions return `Result<(), crate::client::Error>`; use `?` to propagate.
- Startup code returns `anyhow::Result`; config errors use `thiserror` (`ConfigError`).
- Avoid `unwrap()`/`expect()` outside tests. Never `panic!` in handlers.
- The global handler in `src/error.rs` logs the error and sends a friendly message to the user.

## Discord Bot Patterns

**Commands:**

- Place commands in `src/commands/<category>/<command_name>.rs`.
- Define with `#[poise::command(slash_command)]`.
- Expose them from the category's `mod.rs` via `commands()`; categories are aggregated in `commands::all()`.
- Restrict to the bot owner with `check = "is_bot_owner"` (`src/utils/checks.rs`).

**Events:**

- Place events in `src/events/<event_name>.rs` exposing `pub async fn run(...)`.
- Dispatch them in the `match` of `events::handle` (`src/events/mod.rs`) using `serenity::FullEvent`.

**Client:**

- `client::start(config)` builds the Poise framework, registers slash commands globally in `setup` and starts the serenity client.
- Shared state goes in `client::Data` (available as `ctx.data()`); use `Arc`/atomics/`Mutex` for mutable state.
- Gateway intents are defined in `src/client.rs`; add privileged intents (e.g. `MESSAGE_CONTENT`) only when needed.

## Configuration

- Environment variables loaded via `dotenvy` and validated by `Config::from_env()` (`src/helpers/config.rs`).
- Required env vars: `DISCORD_BOT_TOKEN`, `DISCORD_ADMIN_ID`, `DISCORD_APPLICATION_ID`, `DISCORD_PUBLIC_KEY`.

## Logging

- Use `tracing` macros with a scope as target: `info!(target: "client", "...")`, `error!`, `warn!`, `debug!`.
- Logs go to console and to `logs/combined.log` / `logs/error.log` (see `src/helpers/logger.rs`).
- Keep the `WorkerGuard`s returned by `logger::init()` alive in `main`.
