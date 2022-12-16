pub trait FileProvider {
    fn provide_file(&self, path: &str) -> Vec<u8>;
}