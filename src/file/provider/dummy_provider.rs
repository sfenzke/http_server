use super::file_provider::{FileProvider};
use super::error::FileProviderError;

pub struct DummyProvider {}

impl DummyProvider {
    pub fn new() -> Self {
        DummyProvider {}
    }
}

impl FileProvider for DummyProvider {
    fn provide_file(&self, path: &str) -> Result<Vec<u8>, FileProviderError> {
        println!("{}", path);
        Ok(vec![0; 10])
    }
}