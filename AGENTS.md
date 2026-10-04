# AGENTS.md

## Project
`dot` — CLI tool (Rust) for managing dotfiles via symbolic links. Creates
symlinks from a target directory (default: `$HOME`) to files in a source
directory (default: cwd). Backs up existing files before overwriting and can
restore them by replacing symlinks with regular copies.

Rust edition 2024, MSRV 1.88.0. Three production dependencies: `clap`,
`walkdir`, `symlink`. No runtime configuration — CLI flags only.

## Commands
All commands are reachable through the `Makefile`. Run `make help` for the full list.
```bash
make check   # fmt → clippy → test → build (same order as CI — the single verification gate)
make build   # cargo build (debug)
make test    # cargo test
make release # cargo build --release
```

## Hard constraints (MUST)
- **No new dependencies without strong justification.** Prefer a hand-rolled
  solution (ANSI coloring, timestamp computation, symlink probing).
- **Rust edition 2024, MSRV 1.88.0.** Bumping the MSRV requires updating CI,
  `Cargo.toml`'s `rust-version`, and a note in `docs/DECISIONS.md`.
- **No directory symlinks.** Only regular files are linked (see README §
  Technical notes, item 6d).
- **Tests never touch `$HOME`.** All tests use `tempfile::TempDir`.
- **Symlink-capable tests must self-skip** on platforms without symlinks
  (Windows without Developer Mode) — skip, don't fail.
- **No config file, no templates, no DSL.** Configuration via `--source` /
  `--target` flags only.
- **Backward compatibility.** Backup naming (`<file>.bak.YYYY-MM-DD_HH-MM-SS`)
  and the `dot link` command must remain idempotent.
- **Cross-platform.** Build and pass tests on Linux, macOS, Windows.
  OS-specific code behind `#[cfg]`, never separate platform crates.
- **No `unsafe` code.**

## Definition of Done
A feature is done = `make check` is green + CI is green on all matrix targets
+ `--dry-run` is implemented for mutating commands + new commands follow the
`FileOperation` trait + `--version` reports correct hash + README is updated +
integration tests exist in `tests/` for success, failure, and `--dry-run` paths.

"Code written" is not done.

## Work rules (correctness → performance → style)
- **WIP = 1.** One feature at a time. Do not start a second before the first
  passes `make check`.
- **No incidental refactoring** while the main feature is unverified.
- **Atomic commits.** One logical unit = one commit. Rollback must be a single
  `git revert`.
- Before ending a session, make sure `make check` passes.

## Where to find details
- `src/main.rs` — entry point (before dispatching new commands)
- `src/cli_arguments/mod.rs` — clap parser (before adding flags/commands)
- `src/environment/mod.rs` — Environment struct (before changing dir logic)
- `src/handlers/mod.rs` — dispatch to operations (before adding a command)
- `src/handlers/operations/` — FileOperation implementations (before any op change)
- `src/handlers/utils/file_operation.rs` — `FileOperation` trait + `iterate_files()` (before introducing new iteration patterns)
- `src/handlers/utils/logged_operation.rs` — logging decorator (before changing output format)
- `src/log/mod.rs` — ANSI logger (before changing terminal output)
- `tests/common/mod.rs` — test helpers (always consult before writing new tests)
- `tests/` — integration tests per command; primary source of truth for behavior
- `docs/architecture.md` — source tree, design patterns, shell scripts (before structural changes)
- `docs/conventions.md` — coding conventions, CI setup (before changing patterns)
- `docs/DECISIONS.md` — accepted AND rejected decisions with reasons (before proposing an alternative approach)
- `README.md` — user-facing documentation

## Architecture overview
```
main.rs → parse args → build Environment → dispatch to handler → FileOperation
```
See `docs/architecture.md` for the full source tree and design patterns.
