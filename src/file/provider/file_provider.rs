use super::error::FileNotFoundError;

pub trait FileProvider {
    fn provide_file(&self, path: &str) -> Result<Vec<u8>, FileNotFoundError>;
}