use super::file_provider::FileProvider;

pub struct DummyProvider {}

impl DummyProvider {
    pub fn new() -> Self {
        DummyProvider {}
    }
}

impl FileProvider for DummyProvider {
    fn provide_file(&self, path: &str) -> Vec<u8> {
        println!("{}", path);
        vec![0; 10]
    }
}