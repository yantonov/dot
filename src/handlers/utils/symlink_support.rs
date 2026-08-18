use std::path::Path;

const PROBE_LINK_NAME: &str = ".dot-symlink-probe";
const PROBE_LINK_TARGET_NAME: &str = ".dot-symlink-probe-target";

fn unsupported_message(target_directory: &Path, error: &std::io::Error) -> String {
    let hint = if cfg!(windows) {
        " (on Windows this requires Developer Mode to be enabled or the SeCreateSymbolicLinkPrivilege to be granted)"
    } else {
        ""
    };
    format!(
        "cannot create symbolic links in {}: {}{}",
        target_directory.display(),
        error,
        hint
    )
}

// Creating a single symlink up front turns a missing privilege into one error
// before the first file is touched, instead of the same error repeated for
// every file of the tree.
pub fn check_symlink_support(target_directory: &Path) -> Result<(), String> {
    let probe_link_path = target_directory.join(PROBE_LINK_NAME);
    // the probe never points at anything real, so removing it cannot affect
    // whatever else lives in the target directory
    let probe_link_target_path = target_directory.join(PROBE_LINK_TARGET_NAME);

    match std::fs::symlink_metadata(probe_link_path.as_path()) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            std::fs::remove_file(probe_link_path.as_path()).map_err(|e| e.to_string())?;
        }
        Ok(_) => {
            return Err(format!(
                "{} already exists and is not a symbolic link",
                probe_link_path.display()
            ));
        }
        Err(_) => {}
    }

    match symlink::symlink_file(probe_link_target_path.as_path(), probe_link_path.as_path()) {
        Ok(_) => {
            let _ = std::fs::remove_file(probe_link_path.as_path());
            Ok(())
        }
        Err(e) => Err(unsupported_message(target_directory, &e)),
    }
}
