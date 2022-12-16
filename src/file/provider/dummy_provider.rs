use super::file_provider::{FileProvider};
use super::error::FileNotFoundError;

pub struct DummyProvider {}

impl DummyProvider {
    pub fn new() -> Self {
        DummyProvider {}
    }
}

impl FileProvider for DummyProvider {
    fn provide_file(&self, path: &str) -> Result<Vec<u8>, FileNotFoundError> {
        println!("{}", path);
        Ok(vec![0; 10])
    }
}