mod common;

use common::{dot_with_dirs, source_and_target, symlinks_supported};
use std::fs;

fn link_and_produce_a_backup(source: &tempfile::TempDir, target: &tempfile::TempDir) {
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();
    fs::write(target.path().join("bashrc"), "old content").unwrap();

    let status = dot_with_dirs(source, target)
        .arg("link")
        .status()
        .expect("failed to run dot link");
    assert!(status.success());
}

fn backup_file_names(target: &tempfile::TempDir) -> Vec<String> {
    fs::read_dir(target.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("bashrc.bak."))
        .collect()
}

#[test]
fn removes_backup_files() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    link_and_produce_a_backup(&source, &target);
    assert_eq!(backup_file_names(&target).len(), 1);

    let status = dot_with_dirs(&source, &target)
        .args(["backup", "remove"])
        .status()
        .expect("failed to run dot backup remove");
    assert!(status.success());

    assert!(
        backup_file_names(&target).is_empty(),
        "backup file should have been removed"
    );
}

#[test]
fn dry_run_does_not_remove_backup_files() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    link_and_produce_a_backup(&source, &target);
    assert_eq!(backup_file_names(&target).len(), 1);

    let status = dot_with_dirs(&source, &target)
        .args(["--dry-run", "backup", "remove"])
        .status()
        .expect("failed to run dot backup remove --dry-run");
    assert!(status.success());

    assert_eq!(
        backup_file_names(&target).len(),
        1,
        "dry run must not remove the backup file"
    );
}
