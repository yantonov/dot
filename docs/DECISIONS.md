# Architectural decisions

## Decisions log

| Decision | Status | Reason |
|---|---|---|
| No config file, no templates, no DSL | Accepted | Tool is intentionally simple. Configuration via CLI flags only (`--source`, `--target`). |
| No directory symlinks | Accepted | Deliberate design choice — see README § Technical notes. Only regular files are linked. Missing parents created via `create_dir_all`. |
| Hand-rolled ANSI coloring instead of `colored`/`termcolor` crate | Accepted | Minimize dependency footprint. Coloring is trivial (<30 lines). |
| Hand-rolled timestamp (Howard Hinnant's algorithm) instead of `chrono` | Accepted | Timestamp formatting does not justify pulling in a timezone crate with its dependency tree. |
| Symlink tests skip on Windows instead of requiring Developer Mode | Accepted | GitHub-hosted Windows runners don't consistently support symlinks. Skipping is honest. |
| `Result<(), String>` instead of `Box<dyn Error>` | Accepted | Errors are simple strings; no need for error-chain complexity in a CLI tool. |
| MSRV enforced via dedicated CI job | Accepted | Keeps `rust-version` in `Cargo.toml` honest. |
