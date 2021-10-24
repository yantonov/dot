use std::path::{Path, PathBuf};

use walkdir::DirEntry;

use crate::handlers::operations::backup::name_convention::get_backup_file_path;
use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::handlers::utils::file_utils::{target_path};
use crate::util::to_result;

pub struct LinkFileOperation {}

impl LinkFileOperation {
    fn create_backup_file(&self,
                          target_path: &Path,
                          source_path: &Path) -> Result<Option<PathBuf>, String> {
        if !target_path.exists() {
            return Ok(None);
        }
        let link = std::fs::read_link(target_path);
        if link.is_ok() && link.unwrap().as_path() == source_path {
            return Ok(None);
        }

        let backup_file_path = get_backup_file_path(target_path)?;
        let backup_file_path_result = backup_file_path.clone();

        std::fs::copy(target_path, backup_file_path)
            .map(|_| Some(backup_file_path_result))
            .map_err(|e| e.to_string())
    }

    fn create_parent_directory(&self,
                               target_file_path: &Path) -> Result<(), String> {
        let target_file_path_parent_dir = to_result(
            target_file_path.parent(),
            "cannot get parent directory")?;
        if !target_file_path_parent_dir.exists() {
            std::fs::create_dir_all(target_file_path_parent_dir)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

impl FileOperation for LinkFileOperation {
    type Context = FileOperationContext;

    fn call(&self, context: &Self::Context, entry: &DirEntry) -> Result<(), String> {
        let target_file_pathbuf = target_path(context, &entry)?;
        let target_file_path = target_file_pathbuf.as_path();
        let source_file_path = entry.path();

        self.create_parent_directory(&target_file_path)?;

        let backup_file_result = self.create_backup_file(&target_file_path, &source_file_path)?;

        // The goal here is to check whether the file\symlink exists before we try to delete it.
        // This is needed to distinguish the situation when it is impossible to delete the file
        // from the situation that we are trying to delete the nonexistent file
        let metadata = std::fs::symlink_metadata(target_file_path);
        if metadata.is_ok() {
            std::fs::remove_file(target_file_path)
                .map_err(|e| e.to_string())?;
        }
        match symlink::symlink_file(source_file_path, &target_file_path) {
            Ok(_) => Ok(()),
            Err(e) => {
                match backup_file_result {
                    None => {}
                    Some(backup_file) => {
                        std::fs::copy(backup_file.as_path(), target_file_path)
                            .map_err(|e| e.to_string())?;
                    }
                }
                Err(e.to_string())
            }
        }
    }
}