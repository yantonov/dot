mod common;

use std::fs;
use common::{dot_with_dirs, source_and_target, symlinks_supported};

#[test]
fn creates_symlink_pointing_at_source_file() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();

    let status = dot_with_dirs(&source, &target)
        .arg("link")
        .status()
        .expect("failed to run dot");
    assert!(status.success());

    let linked = target.path().join("bashrc");
    let resolved = fs::canonicalize(&linked).expect("linked file should resolve");
    let expected = fs::canonicalize(source.path().join("bashrc")).unwrap();
    assert_eq!(resolved, expected);
}

#[test]
fn creates_nested_directories_and_links_nested_files() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    fs::create_dir_all(source.path().join("config/app")).unwrap();
    fs::write(source.path().join("config/app/settings.toml"), "key = 1").unwrap();

    let status = dot_with_dirs(&source, &target)
        .arg("link")
        .status()
        .expect("failed to run dot");
    assert!(status.success());

    let linked = target.path().join("config/app/settings.toml");
    let resolved = fs::canonicalize(&linked).expect("linked file should resolve");
    let expected = fs::canonicalize(source.path().join("config/app/settings.toml")).unwrap();
    assert_eq!(resolved, expected);
}

#[test]
fn backs_up_existing_file_before_linking() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();
    fs::write(target.path().join("bashrc"), "old content").unwrap();

    let status = dot_with_dirs(&source, &target)
        .arg("link")
        .status()
        .expect("failed to run dot");
    assert!(status.success());

    let backups: Vec<_> = fs::read_dir(target.path()).unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("bashrc.bak."))
        .collect();
    assert_eq!(backups.len(), 1, "expected exactly one backup file, found {:?}", backups);

    let backup_content = fs::read_to_string(target.path().join(&backups[0])).unwrap();
    assert_eq!(backup_content, "old content");

    let linked = fs::canonicalize(target.path().join("bashrc")).unwrap();
    let expected = fs::canonicalize(source.path().join("bashrc")).unwrap();
    assert_eq!(linked, expected);
}

#[test]
fn dry_run_does_not_touch_the_filesystem() {
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();
    fs::write(target.path().join("bashrc"), "old content").unwrap();

    let status = dot_with_dirs(&source, &target)
        .args(["--dry-run", "link"])
        .status()
        .expect("failed to run dot");
    assert!(status.success());

    assert_eq!(fs::read_to_string(target.path().join("bashrc")).unwrap(), "old content",
               "dry run must not overwrite the existing target file");
    assert!(fs::symlink_metadata(target.path().join("bashrc")).unwrap().file_type().is_file(),
            "dry run must not replace the target file with a symlink");

    let backups: Vec<_> = fs::read_dir(target.path()).unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("bashrc.bak."))
        .collect();
    assert!(backups.is_empty(), "dry run must not create a backup file, found {:?}", backups);
}

#[test]
fn running_link_twice_does_not_create_a_second_backup() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();
    fs::write(target.path().join("bashrc"), "old content").unwrap();

    for _ in 0..2 {
        let status = dot_with_dirs(&source, &target)
            .arg("link")
            .status()
            .expect("failed to run dot");
        assert!(status.success());
    }

    let backups: Vec<_> = fs::read_dir(target.path()).unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("bashrc.bak."))
        .collect();
    assert_eq!(backups.len(), 1,
               "re-linking an already-correct symlink should not create another backup, found {:?}",
               backups);
}
