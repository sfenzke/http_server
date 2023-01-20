use super::error::file_provider_error::ErrorType;
use super::file_provider::{FileProvider};
use super::error::FileProviderError;
use std::{fs::read, path::PathBuf};

pub struct SimpleFilesystemProvider {
    base_dir: PathBuf
}

impl SimpleFilesystemProvider {
    pub fn new(base_dir:String) -> Self {
        let mut path = PathBuf::new();
        path.push(base_dir);

        Self {
            base_dir: path
        }
    }
}

impl FileProvider for SimpleFilesystemProvider {
    fn provide_file(&self, file: &str) -> Result<Vec<u8>, FileProviderError> {
        let mut path = self.base_dir.clone();
        path.push(file);

        match read(path) {
            Ok(data) => Ok(data),
            Err(e) => Err(FileProviderError::new(format!("file {} not found", file), ErrorType::FILE_NOT_FOUND))
        }
    }
}