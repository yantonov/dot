use std::path::Path;

use walkdir::DirEntry;

use crate::handlers::file_operation::FileOperation;
use crate::handlers::file_operation_context::FileOperationContext;
use crate::handlers::file_utils::get_relative_file_name;

pub struct UnlinkFileOperation {}

impl FileOperation for UnlinkFileOperation {
    type Context = FileOperationContext;

    fn call(&self, context: &Self::Context, entry: &DirEntry) -> Result<(), String> {
        let file_name = get_relative_file_name(&context.current_directory(), entry)?;

        let home_file_pathbuf = Path::join(Path::new(&context.home()), file_name);
        let home_file_path = home_file_pathbuf.as_path();
        let repository_file_path = entry.path();
        if home_file_path.exists() {
            let link = std::fs::read_link(home_file_path);
            if link.is_ok() {
                if link.unwrap().as_path() == entry.path() {
                    std::fs::remove_file(home_file_path)
                        .map_err(|e| e.to_string())?;
                    std::fs::copy(repository_file_path, home_file_path)
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }
}