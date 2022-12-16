use super::file_provider::{FileProvider};
use super::error::FileNotFoundError;
use std::fs;

pub struct SimpleFilesystemProvider {
    base_dir: String
}

impl SimpleFilesystemProvider {
    pub fn new(base_dir:String) -> Self {
        Self {
            base_dir
        }
    }
}

impl FileProvider for SimpleFilesystemProvider {
    fn provide_file(&self, path: &str) -> Result<Vec<u8>, FileNotFoundError> {
        Ok(vec![0,10])
    }
}