# Coding conventions

- Rust edition 2024, MSRV 1.88.0.
- Functions return `Result<(), String>` (not `Box<dyn Error>`) — errors are
  simple strings composed at the call site.
- No third-party color/logging crate — ANSI escape codes are hand-rolled in
  `log/mod.rs` and only activated when stdout is a terminal.
- `Environment` is a plain struct (not a trait) with two `PathBuf` fields.
- `Arguments` wraps `Opts` — it's not a plain clap struct, providing methods
  like `command()`, `verbose()`, `dry_run()`, and validated
  `source_directory()` / `target_directory()`.
- Tests have their own `common` module with shared setup helpers.

## CI

- **ci.yml**: Push/PR to `master`. Build, fmt, clippy, test on `ubuntu-latest`
  and `macos-latest`. Separate MSRV job. Windows excluded because GitHub-hosted
  Windows runners may not consistently support symlinks.
- **release.yml**: Triggered by tag push. Cross-compiles for 5 targets (linux
  x86_64/aarch64, windows x86_64, macos arm64/x86_64), creates `.tar.gz`
  archives with sha256 checksums, uploads as draft GitHub release.
