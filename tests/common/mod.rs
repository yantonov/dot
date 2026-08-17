use std::fs;
use std::process::Command;
use tempfile::TempDir;

pub fn dot() -> Command {
    Command::new(env!("CARGO_BIN_EXE_dot"))
}

// The whole test suite exercises `--source`/`--target` so it never touches
// the real HOME directory, only these disposable temp dirs.
pub fn source_and_target() -> (TempDir, TempDir) {
    (tempfile::tempdir().expect("source tempdir"),
     tempfile::tempdir().expect("target tempdir"))
}

pub fn dot_with_dirs(source: &TempDir, target: &TempDir) -> Command {
    let mut cmd = dot();
    cmd.args([
        "--source", source.path().to_str().unwrap(),
        "--target", target.path().to_str().unwrap(),
    ]);
    cmd
}

// Creating symlinks requires an elevated privilege on Windows (or Developer
// Mode) and can be unavailable in some sandboxed environments. Tests skip
// themselves rather than fail when the environment can't grant it.
pub fn symlinks_supported() -> bool {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = dir.path().join("source");
    let link = dir.path().join("link");
    fs::write(&source, "probe").expect("write probe file");
    symlink::symlink_file(&source, &link).is_ok()
}
