mod common;

use std::fs;
use common::{dot_with_dirs, source_and_target, symlinks_supported};

#[test]
fn fails_when_symlinks_do_not_exist_yet() {
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();

    let status = dot_with_dirs(&source, &target)
        .arg("check")
        .status()
        .expect("failed to run dot check");
    assert!(!status.success(), "check should fail before linking");
}

#[test]
fn succeeds_after_linking() {
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

    let check_status = dot_with_dirs(&source, &target)
        .arg("check")
        .status()
        .expect("failed to run dot check");
    assert!(check_status.success(), "check should succeed once every file is linked");
}

#[test]
fn fails_when_only_some_files_are_linked() {
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

    // a file added to source after linking has no corresponding symlink yet
    fs::write(source.path().join("vimrc"), "set number").unwrap();

    let check_status = dot_with_dirs(&source, &target)
        .arg("check")
        .status()
        .expect("failed to run dot check");
    assert!(!check_status.success(), "check should fail when a source file has no link");
}
