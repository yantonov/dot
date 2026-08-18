use std::path::{Path, PathBuf};

use walkdir::DirEntry;

use crate::handlers::operations::backup::name_convention::get_backup_file_path;
use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::handlers::utils::file_utils::target_path;

pub struct LinkFileOperation {}

fn get_temporary_link_path(target_file_path: &Path) -> Result<PathBuf, String> {
    let path_str: String = [
        target_file_path.to_str().ok_or("cannot get file name")?,
        ".dot-tmp",
    ]
    .join("");
    Ok(Path::new(&path_str).to_path_buf())
}

impl LinkFileOperation {
    fn needs_backup(&self, target_path: &Path, source_path: &Path) -> bool {
        if !target_path.exists() {
            return false;
        }
        if let Ok(link) = std::fs::read_link(target_path)
            && link.as_path() == source_path
        {
            return false;
        }
        true
    }

    fn create_backup_file(
        &self,
        target_path: &Path,
        source_path: &Path,
    ) -> Result<Option<PathBuf>, String> {
        if !self.needs_backup(target_path, source_path) {
            return Ok(None);
        }

        let backup_file_path = get_backup_file_path(target_path)?;
        let backup_file_path_result = backup_file_path.clone();

        std::fs::copy(target_path, backup_file_path)
            .map(|_| Some(backup_file_path_result))
            .map_err(|e| e.to_string())
    }

    fn describe_plan(
        &self,
        context: &FileOperationContext<'_>,
        target_file_path: &Path,
        source_file_path: &Path,
    ) -> Result<(), String> {
        if self.needs_backup(target_file_path, source_file_path) {
            let backup_file_path = get_backup_file_path(target_file_path)?;
            context.logger().log_dry_run_plan(&format!(
                "would back up {} to {}, then link it to {}",
                target_file_path.display(),
                backup_file_path.display(),
                source_file_path.display()
            ));
        } else if !target_file_path.exists() {
            context.logger().log_dry_run_plan(&format!(
                "would link {} to {}",
                target_file_path.display(),
                source_file_path.display()
            ));
        }
        Ok(())
    }

    fn create_parent_directory(&self, target_file_path: &Path) -> Result<(), String> {
        let target_file_path_parent_dir = target_file_path
            .parent()
            .ok_or("cannot get parent directory")?;
        if !target_file_path_parent_dir.exists() {
            std::fs::create_dir_all(target_file_path_parent_dir).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    // The symlink is built next to the target and only then moved onto it, so that
    // a failure to create it (a missing privilege, for instance) leaves whatever
    // the target already was untouched, instead of deleting it first and having
    // nothing to put back.
    fn create_temporary_link(
        &self,
        target_file_path: &Path,
        source_file_path: &Path,
    ) -> Result<PathBuf, String> {
        let temporary_link_path = get_temporary_link_path(target_file_path)?;
        match std::fs::symlink_metadata(temporary_link_path.as_path()) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                std::fs::remove_file(temporary_link_path.as_path()).map_err(|e| e.to_string())?;
            }
            // anything else at that path was not left there by dot, removing it
            // could destroy a real file
            Ok(_) => {
                return Err(format!(
                    "{} already exists and is not a symbolic link",
                    temporary_link_path.display()
                ));
            }
            Err(_) => {}
        }
        symlink::symlink_file(source_file_path, temporary_link_path.as_path())
            .map(|_| temporary_link_path)
            .map_err(|e| e.to_string())
    }

    fn discard(&self, temporary_link_path: &Path, backup_file_path: &Option<PathBuf>) {
        let _ = std::fs::remove_file(temporary_link_path);
        if let Some(backup_file) = backup_file_path {
            let _ = std::fs::remove_file(backup_file);
        }
    }
}

impl FileOperation for LinkFileOperation {
    fn call(&self, context: &FileOperationContext<'_>, entry: &DirEntry) -> Result<(), String> {
        let target_file_pathbuf = target_path(context, entry)?;
        let target_file_path = target_file_pathbuf.as_path();
        let source_file_path = entry.path();

        if context.dry_run() {
            return self.describe_plan(context, target_file_path, source_file_path);
        }

        self.create_parent_directory(target_file_path)?;

        let temporary_link_path = self.create_temporary_link(target_file_path, source_file_path)?;

        let backup_file_path = match self.create_backup_file(target_file_path, source_file_path) {
            Ok(backup_file_path) => backup_file_path,
            Err(e) => {
                self.discard(temporary_link_path.as_path(), &None);
                return Err(e);
            }
        };

        match std::fs::rename(temporary_link_path.as_path(), target_file_path) {
            Ok(_) => Ok(()),
            Err(e) => {
                self.discard(temporary_link_path.as_path(), &backup_file_path);
                Err(e.to_string())
            }
        }
    }
}
