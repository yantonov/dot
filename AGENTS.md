# AGENTS.md

## Project overview

`dot` is a command-line tool (Rust) that manages dotfiles by creating symbolic
links from a target directory (default: `$HOME`) to files in a source directory
(default: current working directory). It backs up existing files before
overwriting them and can restore them by replacing symlinks with regular copies.

- **Language**: Rust (edition 2024, MSRV 1.88.0)
- **Binary name**: `dot`
- **Repository**: <https://github.com/yantonov/dot>
- **License**: Apache-2.0

## Commands

| Command            | Description                                              |
| ------------------ | -------------------------------------------------------- |
| `dot link`         | Create symlinks in target pointing to source files       |
| `dot unlink`       | Replace symlinks with regular copies of source files     |
| `dot list`         | List all files (recursively) in the source directory     |
| `dot check`        | Verify that every source file has a corresponding symlink |
| `dot backup list`  | List backup files                                        |
| `dot backup remove`| Remove backup files                                      |

Global flags: `--verbose` / `-v`, `--dry-run`, `--source`, `--target`.

## Architecture

```
src/
├── main.rs                 # Entry point: parse args, build Environment, dispatch
├── cli_arguments/mod.rs    # clap derive parser (Opts, Command, Arguments)
├── environment/mod.rs      # Environment struct (source_directory, target_directory)
├── log/mod.rs              # Logger (verbose/non-verbose, ANSI red/green coloring)
└── handlers/
    ├── mod.rs              # Dispatch: link, unlink, list, check, list_backup, remove_backup
    ├── operations/
    │   ├── mod.rs
    │   ├── link_operation.rs         # LinkFileOperation: create symlinks + backup
    │   ├── unlink_operation.rs       # UnlinkFileOperation: replace symlink with copy
    │   ├── list_operation.rs         # ListFileOperation: print source file paths
    │   ├── check_operation.rs        # CheckFileOperation: verify symlinks exist
    │   ├── list_backup_operation.rs  # List backup files for each source entry
    │   ├── remove_backup_operation.rs# Remove backup files
    │   └── backup/
    │       ├── mod.rs
    │       ├── name_convention.rs    # Backup naming: <file>.bak.<YYYY-MM-DD_HH-MM-SS>
    │       └── lister.rs             # Walk target dir to find backup files
    └── utils/
        ├── mod.rs
        ├── file_operation.rs         # FileOperation trait + iterate_files()
        ├── file_operation_context.rs # FileOperationContext (target_dir, source_dir, logger, dry_run)
        ├── file_utils.rs             # target_path(): compute target path from source relative path
        ├── logged_operation.rs       # LoggedOperation decorator: logs [Ok]/[Error] per file
        └── symlink_support.rs        # Probe-based symlink capability check (upfront)

tests/
├── common/mod.rs           # Helpers: dot() cmd, source_and_target(), symlinks_supported()
├── link.rs                 # Integration tests for `dot link`
├── unlink.rs               # Integration tests for `dot unlink`
├── check.rs                # Integration tests for `dot check`
└── backup_remove.rs        # Integration tests for `dot backup remove`
```

### Key design patterns

1. **FileOperation trait**: The core abstraction. Every operation (link, unlink,
   list, check, backup list, backup remove) implements `FileOperation` with a
   single `call(context, entry)` method. `iterate_files()` in
   `file_operation.rs` walks the source directory with `WalkDir`, filters out
   directories, and calls the operation on each file.

2. **LoggedOperation decorator**: Wraps any `FileOperation` to add per-file
   logging (`[Ok]` in green on success, `[Error]` in red on failure). This is
   how operations are used from `handlers/mod.rs`. The decorator also ensures
   that a failure on one file doesn't stop processing of the rest (fold-based
   iteration, not `try_fold`).

3. **FileOperationContext**: A read-only context struct holding target directory,
   source directory, logger reference, and dry_run flag. Passed to every
   `FileOperation::call()`.

4. **Atomic symlink creation**: `LinkFileOperation` creates the symlink at a
   temporary path (`<target>.dot-tmp`) first, then renames it onto the target.
   If the rename fails, the temporary link is cleaned up. This ensures that a
   failure (e.g. permission error) doesn't leave the target in a broken state.

5. **Upfront symlink probe**: Before `dot link` touches any file, it creates and
   deletes a single probe symlink (`.dot-symlink-probe`) in the target directory
   to verify symlink creation is possible. This turns a missing privilege into
   one clear error rather than N repeated errors.

6. **Backup naming**: Backups use the pattern `<filename>.bak.YYYY-MM-DD_HH-MM-SS`
   (UTC). The timestamp is computed using Howard Hinnant's `civil_from_days`
   algorithm (no timezone crate dependency). The `is_backup_file` function
   validates the format with a closure-based pattern matcher.

7. **No directories linked**: Only regular files in the source tree generate
   symlinks. Missing parent directories in the target are created automatically
   (`create_dir_all`). This is intentional — the tool doesn't do "tree folding"
   like GNU Stow.

8. **Idempotent link**: If a target file is already a symlink pointing to the
   canonical source path, `link` skips it without creating a new backup.

9. **MSRV CI**: A dedicated CI job builds and tests with the MSRV (1.88.0)
   to keep `rust-version` in `Cargo.toml` honest.

10. **Git hash in version**: `build.rs` captures `git rev-parse HEAD` at build
    time and embeds it via `env!("GIT_HASH")`, so `dot --version` shows the
    exact commit.

## Build & test

```bash
cargo build
cargo build --release
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Tests use `tempfile::TempDir` for source and target directories and never touch
the real `$HOME`. Tests that require symlink creation call `symlinks_supported()`
and skip themselves when symlinks are unavailable (e.g. on Windows without
Developer Mode).

## CI

- **ci.yml**: Runs on push/PR to `master`. Build, fmt, clippy, test on
  `ubuntu-latest` and `macos-latest`. Separate MSRV job. Windows is excluded
  from CI because GitHub-hosted Windows runners may not consistently support
  symlinks (requires Developer Mode or SeCreateSymbolicLinkPrivilege).
- **release.yml**: Triggered by tag push. Cross-compiles for 5 targets
  (linux x86_64/aarch64, windows x86_64, macos arm64/x86_64), creates `.tar.gz`
  archives with sha256 checksums, uploads as draft GitHub release.

## Dependencies

| Crate      | Purpose                        |
| ---------- | ------------------------------ |
| `clap`     | CLI argument parsing (derive)  |
| `walkdir`  | Recursive directory walking    |
| `symlink`  | Cross-platform symlink creation|

## Hard constraints

- **No new dependencies without strong justification.** The dependency footprint
  is intentionally minimal — three production crates (`clap`, `walkdir`,
  `symlink`) and one dev-dependency (`tempfile`). Before adding a crate, prefer
  a hand-rolled solution (as done for ANSI coloring, timestamp computation, and
  symlink probing).
- **Rust edition 2024, MSRV 1.88.0.** The MSRV is enforced by a dedicated CI
  job. Any change that bumps the required Rust version must update
  `Cargo.toml`'s `rust-version` field and the CI matrix.
- **No directory symlinks.** Only regular files are linked. Missing parent
  directories in the target are created with `create_dir_all`, but the tool
  must never create a symlink pointing at a directory. This is a deliberate
  design choice (see README § Technical notes, item 6d).
- **Must not touch `$HOME` in tests.** All tests use `tempfile::TempDir` for
  both source and target. The real home directory is never affected by the test
  suite.
- **Symlink-capable tests must self-skip when symlinks are unavailable.**
  Any test that calls `symlink::symlink_file` or invokes `dot link` (which
  creates symlinks) must gate on `symlinks_supported()` and skip with a message
  rather than fail.
- **No config file, no templates, no DSL.** The tool is intentionally simple.
  Configuration is done through CLI flags only (`--source`, `--target`). Do
  not introduce configuration files, template engines, or embedded scripting.
- **Backward compatibility.** Existing backup file naming
  (`<file>.bak.YYYY-MM-DD_HH-MM-SS`) and command-line interface must remain
  compatible. `dot link` must remain idempotent (re-linking an already-correct
  symlink is a no-op).
- **Cross-platform.** The tool must build and pass tests on Linux, macOS, and
  Windows (symlink tests skip on Windows where symlinks require elevated
  privileges). OS-specific code paths must be behind `cfg` attributes, not
  separate platform crates.
- **No `unsafe` code.**

## Definition of done

A change is ready when:

- [ ] `cargo build` and `cargo build --release` succeed with no warnings
- [ ] `cargo fmt --check` passes (standard `rustfmt`, no custom config)
- [ ] `cargo clippy --all-targets -- -D warnings` passes with no warnings
- [ ] `cargo test` passes on the developer's platform (symlink-skipping tests
      are acceptable on Windows)
- [ ] CI (`.github/workflows/ci.yml`) is green on all matrix targets:
  `ubuntu-latest`, `macos-latest`, and the MSRV job
- [ ] `--dry-run` is implemented for any new command that mutates the filesystem
      and produces a faithful description of what would happen
- [ ] New commands follow the `FileOperation` trait pattern and are wrapped in
      `LoggedOperation` for consistent per-file logging
- [ ] Any new CLI flags are added to `cli_arguments/mod.rs` with a `clap`
      `#[clap(help = "...")]` doc string
- [ ] `dot --version` continues to report the correct version and git hash
      (`build.rs` embeds `GIT_HASH` at compile time)
- [ ] README is updated if the user-facing behavior changes (new command,
      new flag, changed output)
- [ ] Integration tests exist in `tests/` for new commands, covering both
      success and failure paths, as well as `--dry-run`

## Coding conventions

- Rust edition 2024
- Functions return `Result<(), String>` (not `Box<dyn Error>`) — errors are
  simple strings composed at the call site
- No third-party color/logging crate — ANSI escape codes are hand-rolled in
  `log/mod.rs` and only activated when stdout is a terminal
- `Environment` is a plain struct (not a trait) with two `PathBuf` fields
- `Arguments` wraps `Opts` — it's not a plain clap struct, providing
  methods like `command()`, `verbose()`, `dry_run()`, and validated
  `source_directory()` / `target_directory()`
- Tests have their own `common` module with shared setup helpers
