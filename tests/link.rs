mod common;

use common::{dot_with_dirs, source_and_target, symlinks_supported};
use std::fs;

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

    let backups: Vec<_> = fs::read_dir(target.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("bashrc.bak."))
        .collect();
    assert_eq!(
        backups.len(),
        1,
        "expected exactly one backup file, found {:?}",
        backups
    );

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

    assert_eq!(
        fs::read_to_string(target.path().join("bashrc")).unwrap(),
        "old content",
        "dry run must not overwrite the existing target file"
    );
    assert!(
        fs::symlink_metadata(target.path().join("bashrc"))
            .unwrap()
            .file_type()
            .is_file(),
        "dry run must not replace the target file with a symlink"
    );

    let backups: Vec<_> = fs::read_dir(target.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("bashrc.bak."))
        .collect();
    assert!(
        backups.is_empty(),
        "dry run must not create a backup file, found {:?}",
        backups
    );
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

    let backups: Vec<_> = fs::read_dir(target.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("bashrc.bak."))
        .collect();
    assert_eq!(
        backups.len(),
        1,
        "re-linking an already-correct symlink should not create another backup, found {:?}",
        backups
    );
}

#[test]
fn does_not_leave_temporary_files_behind() {
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

    let leftovers: Vec<_> = fs::read_dir(target.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".dot-tmp"))
        .collect();
    assert!(
        leftovers.is_empty(),
        "no temporary file should survive a successful link, found {:?}",
        leftovers
    );
}

// A file sitting on the temporary path is the one link failure that can be
// staged on every platform - it stands in for the environment where creating a
// symlink is not permitted at all.
#[test]
fn failed_link_keeps_the_existing_symlink() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();
    let unrelated_file = source.path().join("unrelated");
    fs::write(&unrelated_file, "somewhere else").unwrap();
    symlink::symlink_file(&unrelated_file, target.path().join("bashrc")).unwrap();
    fs::write(target.path().join("bashrc.dot-tmp"), "in the way").unwrap();

    let status = dot_with_dirs(&source, &target)
        .arg("link")
        .status()
        .expect("failed to run dot");
    assert!(!status.success());

    let linked = fs::canonicalize(target.path().join("bashrc"))
        .expect("the existing symlink should still resolve");
    assert_eq!(linked, fs::canonicalize(&unrelated_file).unwrap());
}

// The blocked temporary path makes any attempt to relink fail, so the run can
// only succeed when an already-linked file is left alone.
#[test]
fn already_linked_file_is_left_alone() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();
    // dot canonicalizes the directories it is given, and the temp directory is
    // reached through a symlink on macOS, so only a canonical link target is
    // the one dot would have written itself
    let source_file = fs::canonicalize(source.path().join("bashrc")).unwrap();
    symlink::symlink_file(&source_file, target.path().join("bashrc")).unwrap();
    fs::write(target.path().join("bashrc.dot-tmp"), "in the way").unwrap();

    let status = dot_with_dirs(&source, &target)
        .arg("link")
        .status()
        .expect("failed to run dot");
    assert!(status.success());

    let linked = fs::canonicalize(target.path().join("bashrc"))
        .expect("the existing symlink should still resolve");
    assert_eq!(linked, fs::canonicalize(&source_file).unwrap());
}

#[test]
fn failed_link_keeps_the_existing_file_and_leaves_no_backup() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();
    fs::write(target.path().join("bashrc"), "old content").unwrap();
    fs::write(target.path().join("bashrc.dot-tmp"), "in the way").unwrap();

    let status = dot_with_dirs(&source, &target)
        .arg("link")
        .status()
        .expect("failed to run dot");
    assert!(!status.success());

    assert_eq!(
        fs::read_to_string(target.path().join("bashrc")).unwrap(),
        "old content",
        "a failed link must not touch the existing target file"
    );

    let backups: Vec<_> = fs::read_dir(target.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("bashrc.bak."))
        .collect();
    assert!(
        backups.is_empty(),
        "a failed link must not leave a backup file behind, found {:?}",
        backups
    );
}

#[test]
fn probing_for_symlink_support_leaves_nothing_behind() {
    if !symlinks_supported() {
        eprintln!("skipping: symlinks not supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();

    for args in [vec!["--dry-run", "link"], vec!["link"]] {
        let status = dot_with_dirs(&source, &target)
            .args(args)
            .status()
            .expect("failed to run dot");
        assert!(status.success());

        let probes: Vec<_> = fs::read_dir(target.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|name| name.starts_with(".dot-"))
            .collect();
        assert!(probes.is_empty(), "found {:?}", probes);
    }
}

// The counterpart of every skipped test above: this one only runs where
// symlinks are unavailable, which is the environment the check exists for.
#[test]
fn link_reports_missing_symlink_support_and_changes_nothing() {
    if symlinks_supported() {
        eprintln!("skipping: symlinks are supported in this environment");
        return;
    }
    let (source, target) = source_and_target();
    fs::write(source.path().join("bashrc"), "export FOO=1").unwrap();
    fs::write(target.path().join("bashrc"), "old content").unwrap();

    let output = dot_with_dirs(&source, &target)
        .arg("link")
        .output()
        .expect("failed to run dot");
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("cannot create symbolic links"),
        "expected a symlink support error, got {:?}",
        String::from_utf8_lossy(&output.stdout)
    );

    let entries: Vec<_> = fs::read_dir(target.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        entries,
        vec!["bashrc".to_string()],
        "the target directory must be left as it was"
    );
    assert_eq!(
        fs::read_to_string(target.path().join("bashrc")).unwrap(),
        "old content"
    );
}
