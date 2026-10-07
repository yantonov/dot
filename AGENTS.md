# AGENTS.md

## Project
`dot` — Rust CLI that manages dotfiles via symbolic links. Links regular files
from a source directory (default: cwd) into a target directory (default:
`$HOME`), backs up existing files before overwriting, and restores them by
replacing symlinks with regular copies.

Flow: `main.rs` → parse args → build `Environment` → dispatch to handler → `FileOperation`.

## Constraints
- **Dependencies stay at `clap`, `walkdir`, `symlink`.** Hand-roll the rest
  (ANSI coloring, timestamps, symlink probing); a new crate needs a strong
  justification.
- **Edition 2024, MSRV 1.88.0.** An MSRV bump updates CI, `Cargo.toml`
  `rust-version`, and `docs/DECISIONS.md`.
- **Files only.** Link regular files; parent directories are created (README
  § Tecnhical remarks, item 3).
- **Tests live in `tempfile::TempDir`.** `$HOME` stays untouched.
- **Symlink tests self-skip** where symlinks are unavailable (Windows without
  Developer Mode).
- **Flags configure everything:** `--source` / `--target` only.
- **Idempotent `dot link`;** backup names stay `<file>.bak.YYYY-MM-DD_HH-MM-SS`.
- **Cross-platform:** Linux, macOS, Windows. OS-specific code sits behind
  `#[cfg]` in this crate.
- **Safe Rust only.**

## Done
Done = `make check` is green (fmt → clippy → test → build, the CI order) AND
CI is green on every matrix target AND every item holds:
- mutating commands implement `--dry-run`
- new commands implement `FileOperation`
- `--version` reports the correct hash
- README is updated
- `tests/` covers success, failure, and `--dry-run` paths

Code written is not done. Run `make check` before ending a session.

## Work rules
- **WIP = 1:** finish a feature (green `make check`) before starting the next.
- **Scope:** refactor only after the feature is verified.
- **One logical unit per commit,** so rollback is a single `git revert`.

## Pointers
Read the file before the action it names.
- Adding a command → `src/main.rs`, `src/handlers/mod.rs`, `src/cli_arguments/mod.rs` (flags too)
- Changing directory logic → `src/environment/mod.rs`
- Changing an operation → `src/handlers/operations/`
- New iteration pattern → `src/handlers/utils/file_operation.rs` (`FileOperation`, `iterate_files()`)
- Changing output format → `src/handlers/utils/logged_operation.rs`, `src/log/mod.rs`
- Writing a test → `tests/common/mod.rs` first; `tests/` per command is the source of truth for behavior
- Structural change → `docs/architecture.md`
- Changing a pattern or CI → `docs/conventions.md`
- Proposing an alternative approach → `docs/DECISIONS.md` (accepted and rejected, with reasons)
- User-facing change → `README.md`
