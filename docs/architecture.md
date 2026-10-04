# Architecture

## Source tree

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

## Key design patterns

1. **FileOperation trait**: Every operation (link, unlink, list, check, backup
   list, backup remove) implements `FileOperation` with a single `call(context,
   entry)` method. `iterate_files()` walks the source directory with `WalkDir`,
   filters out directories, and calls the operation on each file.

2. **LoggedOperation decorator**: Wraps any `FileOperation` to add per-file
   logging (`[Ok]` in green on success, `[Error]` in red on failure). The
   decorator also ensures that a failure on one file doesn't stop processing of
   the rest (fold-based iteration, not `try_fold`).

3. **FileOperationContext**: Read-only struct holding target directory, source
   directory, logger reference, and dry_run flag. Passed to every
   `FileOperation::call()`.

4. **Atomic symlink creation**: `LinkFileOperation` creates the symlink at a
   temporary path (`<target>.dot-tmp`) first, then renames it onto the target.
   If the rename fails, the temporary link is cleaned up.

5. **Upfront symlink probe**: Before `dot link` touches any file, it creates and
   deletes a single probe symlink (`.dot-symlink-probe`) in the target directory
   to verify symlink creation is possible — one clear error instead of N
   repeated ones.

6. **Backup naming**: `<filename>.bak.YYYY-MM-DD_HH-MM-SS` (UTC). Timestamp
   computed using Howard Hinnant's `civil_from_days` algorithm — no timezone
   crate dependency.

7. **No directories linked**: Only regular files generate symlinks. Missing
   parent directories in the target are created with `create_dir_all`. No "tree
   folding" like GNU Stow.

8. **Idempotent link**: If a target file is already a symlink pointing to the
   canonical source path, `link` skips it without creating a new backup.

9. **Git hash in version**: `build.rs` captures `git rev-parse HEAD` at build
   time and embeds it via `env!("GIT_HASH")`, so `dot --version` shows the exact
   commit.

## Shell scripts

| Script | Purpose |
|---|---|
| `bin/dev/tag-release.sh` | Version bump, commit, tag (called by `make tag-release`) |
| `bin/install/install-from-source.sh` | Copy release binary to `~/.local/bin` (`make install`) |
| `bin/install/install.sh` | Public installer (end users `curl` it from GitHub) |
| `bin/install/download.sh` | Download + verify + unpack binary (called by `install.sh`) |
