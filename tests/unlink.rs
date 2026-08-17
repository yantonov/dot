mod common;

use common::{dot_with_dirs, source_and_target, symlinks_supported};
use std::fs;

#[test]
fn replaces_symlink_with_a_regular_copy_of_the_source_file() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();

    let link_status = dot_with_dirs(&source, &target)
        .arg("link")
        .status()
        .expect("failed to run dot link");
    assert!(link_status.success());

    let unlink_status = dot_with_dirs(&source, &target)
        .arg("unlink")
        .status()
        .expect("failed to run dot unlink");
    assert!(unlink_status.success());

    let linked = target.path().join("bashrc");
    assert!(
        fs::symlink_metadata(&linked).unwrap().file_type().is_file(),
        "target should be a regular file after unlink, not a symlink"
    );
    assert_eq!(fs::read_to_string(&linked).unwrap(), "export FOO=1");

    // source file itself must be untouched by unlink
    assert_eq!(
        fs::read_to_string(source.path().join("bashrc")).unwrap(),
        "export FOO=1"
    );
}

#[test]
fn dry_run_leaves_the_symlink_in_place() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();

    let link_status = dot_with_dirs(&source, &target)
        .arg("link")
        .status()
        .expect("failed to run dot link");
    assert!(link_status.success());

    let dry_run_status = dot_with_dirs(&source, &target)
        .args(["--dry-run", "unlink"])
        .status()
        .expect("failed to run dot unlink --dry-run");
    assert!(dry_run_status.success());

    let linked = target.path().join("bashrc");
    let resolved = fs::canonicalize(&linked).expect("target should still be a symlink");
    let expected = fs::canonicalize(source.path().join("bashrc")).unwrap();
    assert_eq!(
        resolved, expected,
        "dry run must not replace the symlink with a regular file"
    );
}

#[test]
fn does_nothing_when_target_was_never_linked() {
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();

    let status = dot_with_dirs(&source, &target)
        .arg("unlink")
        .status()
        .expect("failed to run dot unlink");
    assert!(status.success());
    assert!(!target.path().join("bashrc").exists());
}
