pub trait FileProvider {
    fn provide_file(&self, path: String) -> Vec<u8>;
}

pub struct DummyProvider {}

impl FileProvider for DummyProvider {
    fn provide_file(&self, path: String) -> Vec<u8> {
        println!("DummyFileProvider: {}", path);
        vec![0; 10]
    }
}

impl DummyProvider {
    pub fn new() -> Self {
        DummyProvider {}
    }
}