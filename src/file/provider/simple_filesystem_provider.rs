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
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Err(FileProviderError::new(format!("file {} not found", file), ErrorType::FileNotFound)),
                std::io::ErrorKind::PermissionDenied => Err(FileProviderError::new(format!("missing read permission for {}", file), ErrorType::ReadError)),
                _ => Err(FileProviderError::new(format!("unknown IO Error"), ErrorType::ReadError)),
            }
        }
    }
}