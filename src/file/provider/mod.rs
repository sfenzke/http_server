pub trait FileProvider {
    fn provide_file(&self, path: &str) -> &[u8];
}